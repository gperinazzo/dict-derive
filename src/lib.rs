#![recursion_limit = "256"]
extern crate proc_macro;
extern crate syn;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::export::{Span, TokenStream2};
use syn::spanned::Spanned;

use syn::{Data, DeriveInput, Field, Ident};

fn is_option(ty: &syn::Type) -> bool {
    let path = match *ty {
        syn::Type::Path(ref p) if p.qself.is_none() => &p.path,
        _ => return false,
    };

    path.segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .any(|x| x.as_str() == "Option")
}

fn map_extraction(field: &Field) -> TokenStream2 {
    let ident = match &field.ident {
        Some(i) => i,
        None => {
            return syn::Error::new(field.span(), "Unnamed fields are not supported")
                .to_compile_error()
        }
    };

    let name = ident.to_string();

    let function = if is_option(&field.ty) {
        Ident::new("extract_optional", field.ty.span())
    } else {
        Ident::new("extract_required", field.ty.span())
    };

    quote_spanned! {ident.span()=>
        #ident: #function(dict, #name)?
    }
}

fn extraction_functions() -> syn::export::TokenStream2 {
    quote! {
        fn map_exception(name: &str, _: PyErr) -> PyErr {
            PyErr::new::<TypeError, _>(format!("Unable to convert key: {}", name))
        }

        fn extract_required<'a, T>(dict: &'a PyDict, name: &str) -> PyResult<T>
        where T: FromPyObject<'a>{
            match dict.get_item(name) {
                Some(v) => FromPyObject::extract(&v)
                    .map_err(|err| map_exception(name, err)),
                None => Err(PyErr::new::<ValueError, _>(format!("Missing required key: {}", name)))
            }
        }

        fn extract_optional<'a, T>(dict: &'a PyDict, name: &str) -> PyResult<std::option::Option<T>>
        where T: FromPyObject<'a>{
            match dict.get_item(name) {
                Some(v) => FromPyObject::extract(&v)
                    .map_err(|err| map_exception(name, err)),
                None => Ok(None),
            }
        }
    }
}

fn generate_error(span: Span, message: &str) -> TokenStream {
    TokenStream::from(syn::Error::new(span, message).to_compile_error())
}

#[proc_macro_derive(FromPyObject)]
pub fn derive_from_py_object(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let struct_data = match ast.data {
        Data::Struct(s) => s,
        Data::Enum(e) => {
            return generate_error(e.enum_token.span, "Deriving enums is not supported")
        }
        Data::Union(u) => {
            return generate_error(u.union_token.span, "Deriving unions is not supported")
        }
    };

    let extractions = struct_data.fields.into_iter().map(map_extraction);

    let name = ast.ident;
    let (_, ty_generics, where_clause) = ast.generics.split_for_impl();

    let functions = extraction_functions();

    let tokens = quote! {
        impl<'source> ::pyo3::FromPyObject<'source> for #name #ty_generics #where_clause {
            fn extract(obj: &'source ::pyo3::types::PyAny) -> ::pyo3::PyResult<Self> {
                use ::pyo3::{FromPyObject, PyErr, PyResult};
                use ::pyo3::exceptions::{ValueError, TypeError};
                use ::pyo3::types::PyDict;
                let dict = <pyo3::types::PyDict as ::pyo3::PyTryFrom>::try_from(obj)
                    .map_err(
                        |_| PyErr::new::<TypeError, _>("Invalid type to convert, expected dict")
                    )?;

                #functions

                ::std::result::Result::Ok(#name {
                    #(#extractions),*
                })

            }
        }

    };

    TokenStream::from(tokens)
}
