#![recursion_limit = "256"]
extern crate proc_macro;
extern crate syn;

mod from;

use from::from_impl;
use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(FromPyObject)]
pub fn derive_from_py_object(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    TokenStream::from(from_impl(ast))
}
