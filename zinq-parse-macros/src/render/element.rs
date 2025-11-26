use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

use crate::parse::Input;

pub fn element(input: &Input, target: &mut syn::Item) -> Result<TokenStream, syn::Error> {
    match target {
        syn::Item::Struct(item) => struct_element(input, item),
        item => Err(syn::Error::new(
            item.span(),
            "[zinq::parse] unsupported type",
        )),
    }
}

pub fn struct_element(
    input: &Input,
    target: &mut syn::ItemStruct,
) -> Result<TokenStream, syn::Error> {
    let name = &target.ident;
    let context_type = match input.context() {
        Some(ty) => quote!(#ty),
        None => quote!(::zinq_parse::contexts::StructContext<proc_macro2::TokenStream>),
    };

    Ok(quote! {
        #target

        impl ::zinq_parse::Element for #name {
            type Context = #context_type;
            type Output = proc_macro2::TokenStream;

            fn render(&self, context: &mut Self::Context) -> Result<Self::Output, ::zinq_parse::Error> {
                Ok(::quote::quote! {

                })
            }
        }
    })
}
