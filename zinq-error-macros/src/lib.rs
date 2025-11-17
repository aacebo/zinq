mod reflect_enum;

use proc_macro::TokenStream;

#[proc_macro_derive(Error, attributes(error))]
pub fn derive_error(tokens: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(tokens as syn::DeriveInput);

    return match &input.data {
        syn::Data::Enum(ty) => reflect_enum::derive(&input, ty),
        _ => quote::quote!(compile_error!("unsupported Error type")),
    }
    .into();
}
