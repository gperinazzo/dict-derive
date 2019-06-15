use quote::{quote, quote_spanned};
use syn::export::TokenStream2;
use syn::spanned::Spanned;

use syn::{Data, DeriveInput, Field};

fn map_fields(field: &Field) -> TokenStream2 {
    let ident = match &field.ident {
        Some(i) => i,
        None => {
            return syn::Error::new(field.span(), "Unnamed fields are not supported")
                .to_compile_error()
        }
    };

    let name = ident.to_string();

    // TODO: find a way to do this without the .expect
    quote_spanned! {field.ty.span()=>
        dict.set_item(#name, IntoPyObject::into_object(self.#ident, py)).expect("Failed to set_item on dict");
    }
}

pub fn into_impl(ast: DeriveInput) -> TokenStream2 {
    let struct_data = match ast.data {
        Data::Struct(s) => s,
        Data::Enum(e) => {
            return syn::Error::new(e.enum_token.span, "Deriving enums is not supported")
                .to_compile_error();
        }
        Data::Union(u) => {
            return syn::Error::new(u.union_token.span, "Deriving unions is not supported")
                .to_compile_error();
        }
    };

    let field_setters = struct_data.fields.into_iter().map(map_fields);

    let name = ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    quote! {
        impl #impl_generics ::pyo3::IntoPyObject for #name #ty_generics #where_clause {
            fn into_object(self, py: ::pyo3::Python) -> ::pyo3::PyObject {
                use ::pyo3::{IntoPyObject, PyErr, PyResult};
                use ::pyo3::exceptions::{ValueError, TypeError};
                use ::pyo3::types::PyDict;
                let dict = PyDict::new(py);
                #(#field_setters);*

                dict.into()
            }
        }

    }
}
