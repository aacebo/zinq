mod parse;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn zinq_derive(args: TokenStream, tokens: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(args as parse::DeriveInput);
    let target = syn::parse_macro_input!(tokens as syn::Item);

    return quote!(#target).into();
}

#[proc_macro_attribute]
pub fn zinq_attribute(args: TokenStream, tokens: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(args as parse::DeriveInput);
    let target = syn::parse_macro_input!(tokens as syn::Item);

    return quote!(#target).into();
}
