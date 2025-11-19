mod params;
mod template;
mod variant;

pub(crate) use params::*;

use proc_macro::TokenStream;
use quote::quote;

use crate::template::Template;

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
    let variant_attributes = data
        .variants
        .iter()
        .map(|variant| {
            let variant_attr = &variant
                .attrs
                .iter()
                .find(|attribute| attribute.path().is_ident("error"));

            let variant_params = match variant_attr {
                None => None,
                Some(attr) => match attr.parse_args::<crate::Params>() {
                    Err(err) => return err.to_compile_error(),
                    Ok(v) => Some(v),
                },
            };

            let variant_error_template_result = match &variant_params {
                None => Template::parse(""),
                Some(p) => match &p.message {
                    None => Template::parse(""),
                    Some(v) => Template::parse(&v),
                },
            };

            let template = match variant_error_template_result {
                Err(err) => return err.to_compile_error(),
                Ok(res) => res,
            };

            let attributes = template
                .arguments
                .iter()
                .map(|arg| {
                    quote! {}
                })
                .collect::<Vec<_>>();

            quote! {
                #(#attributes)*
            }
        })
        .collect::<Vec<_>>();

    return quote! {
        #display
        #error
        #to_error
        #(#variant_attributes)*
    };
}
