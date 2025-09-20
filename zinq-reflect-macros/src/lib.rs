mod reflect_enum;
mod reflect_struct;

use reflect_enum::*;
use reflect_struct::*;

use proc_macro::TokenStream;

#[proc_macro_derive(Reflect)]
pub fn reflect(tokens: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(tokens as syn::DeriveInput);

    return match &input.data {
        syn::Data::Struct(ty) => reflect_struct(&input, ty),
        syn::Data::Enum(ty) => reflect_enum(&input, ty),
        _ => panic!("unsupported Reflect type '{}'", &input.ident),
    };
}
