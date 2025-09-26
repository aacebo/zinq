mod parse;
mod reflect_enum;
mod reflect_field;
mod reflect_mod;
mod reflect_struct;
mod reflect_trait;
mod reflect_visibility;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Reflect, attributes(reflect))]
pub fn derive_reflect(tokens: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(tokens as syn::DeriveInput);

    return match &input.data {
        syn::Data::Struct(ty) => reflect_struct::derive(&input, ty),
        syn::Data::Enum(ty) => reflect_enum::derive(&input, ty),
        _ => quote::quote!(compile_error!("unsupported Reflect type")),
    }
    .into();
}

#[proc_macro_attribute]
pub fn reflect(_attr_tokens: TokenStream, item_tokens: TokenStream) -> TokenStream {
    let mut item = syn::parse_macro_input!(item_tokens as syn::Item);

    return match reflect_item(&mut item) {
        None => quote!(compile_error!("invalid reflect type")),
        Some(v) => v,
    }
    .into();
}

fn reflect_item(item: &mut syn::Item) -> Option<proc_macro2::TokenStream> {
    return match item {
        syn::Item::Mod(v) => Some(reflect_mod::attr(v)),
        syn::Item::Trait(v) => Some(reflect_trait::attr(v)),
        syn::Item::Struct(v) => Some(reflect_struct::attr(v)),
        syn::Item::Enum(v) => Some(reflect_enum::attr(v)),
        _ => None,
    };
}

fn reflect_ty(item: &mut syn::Item) -> Option<proc_macro2::TokenStream> {
    return match item {
        syn::Item::Mod(v) => Some(reflect_mod::ty(v)),
        syn::Item::Trait(v) => Some(reflect_trait::ty(v)),
        syn::Item::Struct(v) => Some(reflect_struct::ty(v)),
        syn::Item::Enum(v) => Some(reflect_enum::ty(v)),
        _ => None,
    };
}
