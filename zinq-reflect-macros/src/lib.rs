mod reflect_enum;
mod reflect_struct;
mod reflect_trait;
mod reflect_visibility;

use proc_macro::TokenStream;

#[proc_macro_derive(Reflect)]
pub fn derive(tokens: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(tokens as syn::DeriveInput);

    return match &input.data {
        syn::Data::Struct(ty) => reflect_struct::derive(&input, ty),
        syn::Data::Enum(ty) => reflect_enum::derive(&input, ty),
        _ => panic!("unsupported Reflect type '{}'", &input.ident),
    }
    .into();
}

#[proc_macro_attribute]
pub fn reflect(_attr_tokens: TokenStream, item_tokens: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(item_tokens as syn::Item);

    return match &item {
        syn::Item::Trait(v) => reflect_trait::attr(v),
        _ => panic!("unsupported reflect type"),
    }
    .into();
}
