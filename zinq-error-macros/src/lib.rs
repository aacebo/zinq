mod params;
mod variant;

pub(crate) use params::*;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Error, attributes(error))]
pub fn derive_error(tokens: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(tokens as syn::DeriveInput);

    return match &input.data {
        syn::Data::Enum(ty) => render(&input, ty),
        _ => quote::quote!(compile_error!("unsupported Error type")),
    }
    .into();
}

fn render(input: &syn::DeriveInput, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let display = variant::display::render(input, data);
    let error = variant::error::render(input, data);
    let to_error = variant::to_error::render(input, data);

    return quote! {
        #display
        #error
        #to_error
    };
}
