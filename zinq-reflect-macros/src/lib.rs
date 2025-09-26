mod parse;
mod reflect_enum;
mod reflect_field;
mod reflect_meta;
mod reflect_mod;
mod reflect_struct;
mod reflect_trait;
mod reflect_visibility;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parser;

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
pub fn reflect(attrs: TokenStream, item_tokens: TokenStream) -> TokenStream {
    let mut item = syn::parse_macro_input!(item_tokens as syn::Item);

    return match reflect_item(attrs, &mut item) {
        None => quote!(compile_error!("invalid reflect type")),
        Some(v) => v,
    }
    .into();
}

fn reflect_item(attrs: TokenStream, item: &mut syn::Item) -> Option<proc_macro2::TokenStream> {
    let mut pairs = vec![];
    let parser = syn::meta::parser(|meta| {
        pairs.push(parse::meta_data_item(meta));
        Ok(())
    });

    if let Err(err) = &parser.parse(attrs) {
        return err.to_compile_error().into();
    }

    let meta = quote!(::zinq_reflect::MetaData::from([#(#pairs,)*]));

    return match item {
        syn::Item::Mod(v) => Some(reflect_mod::attr(meta, v)),
        syn::Item::Trait(v) => Some(reflect_trait::attr(v)),
        syn::Item::Struct(v) => Some(reflect_struct::attr(v)),
        syn::Item::Enum(v) => Some(reflect_enum::attr(v)),
        _ => None,
    };
}

fn reflect_ty(item: &mut syn::Item) -> Option<proc_macro2::TokenStream> {
    return match item {
        syn::Item::Mod(v) => Some(reflect_mod::ty(quote!(), v)),
        syn::Item::Trait(v) => Some(reflect_trait::ty(v)),
        syn::Item::Struct(v) => Some(reflect_struct::ty(v)),
        syn::Item::Enum(v) => Some(reflect_enum::ty(v)),
        _ => None,
    };
}
