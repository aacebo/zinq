mod variant_params;

pub(crate) use variant_params::*;

use proc_macro::TokenStream;
use quote::{format_ident, quote};

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
    let name = &input.ident;
    let variants_to_error = data
        .variants
        .iter()
        .map(|variant| {
            let variant_ident = &variant.ident;
            let variant_attr = &variant
                .attrs
                .iter()
                .find(|attribute| attribute.path().is_ident("error"));

            let variant_params = match variant_attr {
                None => None,
                Some(attr) => match attr.parse_args::<crate::VariantParams>() {
                    Err(err) => return err.to_compile_error(),
                    Ok(v) => Some(v),
                },
            };

            let variant_fields = match &variant.fields {
                syn::Fields::Unit => vec![],
                syn::Fields::Named(fields) => fields
                    .named
                    .iter()
                    .map(|field| {
                        let field_ident = &field.ident;
                        quote!(#field_ident)
                    })
                    .collect::<Vec<_>>(),
                syn::Fields::Unnamed(fields) => fields
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(i, _)| {
                        let field_ident = format_ident!("p{}", i);
                        quote!(#field_ident)
                    })
                    .collect::<Vec<_>>(),
            };

            let variant_error_name = match &variant_params {
                None => quote!(stringify!(#variant_ident)),
                Some(p) => match &p.name {
                    None => quote!(stringify!(#variant_ident)),
                    Some(v) => quote!(&#v),
                },
            };

            let variant_error_code = match &variant_params {
                None => quote!(None),
                Some(p) => match &p.code {
                    None => quote!(None),
                    Some(v) => quote!(Some(#v)),
                },
            };

            let variant_error_message = match &variant_params {
                None => quote!(None),
                Some(p) => match &p.message {
                    None => quote!(None),
                    Some(v) => quote!(Some(#v)),
                },
            };

            let variant_error = quote! {
                let mut builder = ::zinq_error::Error::new()
                    .with_path(&String::from(module_path!()))
                    .with_name(#variant_error_name);

                if let Some(code) = #variant_error_code {
                    builder = builder.with_code(code);
                }

                if let Some(message) = #variant_error_message {
                    builder = builder.with_message(message);
                }

                builder.build()
            };

            if variant_fields.is_empty() {
                return quote!(Self::#variant_ident => {
                    #variant_error
                });
            }

            if variant_fields.len() == 1 {
                return quote!(Self::#variant_ident(v) => {
                    #variant_error
                });
            }

            quote! {
                Self::#variant_ident(#(#variant_fields,)*) => {
                    #variant_error
                }
            }
        })
        .collect::<Vec<_>>();

    let variants_write = data
        .variants
        .iter()
        .map(|variant| {
            let variant_ident = &variant.ident;

            let variant_fields = match &variant.fields {
                syn::Fields::Unit => vec![],
                syn::Fields::Named(fields) => fields
                    .named
                    .iter()
                    .map(|field| {
                        let field_ident = &field.ident;
                        quote!(#field_ident)
                    })
                    .collect::<Vec<_>>(),
                syn::Fields::Unnamed(fields) => fields
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(i, _)| {
                        let field_ident = format_ident!("p{}", i);
                        quote!(#field_ident)
                    })
                    .collect::<Vec<_>>(),
            };

            if variant_fields.is_empty() {
                return quote! {
                    Self::#variant_ident => write!(f, "{}", stringify!(#variant_ident))
                };
            }

            if variant_fields.len() == 1 {
                return quote! {
                    Self::#variant_ident(v) => write!(f, "{}", v)
                };
            }

            quote! {
                Self::#variant_ident(#(#variant_fields,)*) => {
                    write!(f, "{}", (#(#variant_fields,)*))
                }
            }
        })
        .collect::<Vec<_>>();

    return quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                return match self {
                    #(#variants_write,)*
                };
            }
        }

        impl std::error::Error for #name {
            fn source(&self) -> Option<&(dyn ::std::error::Error + 'static)> {
                return None;
            }
        }

        impl ::zinq_error::ToError for #name {
            fn to_error(&self) -> ::zinq_error::Error {
                return match self {
                    #(#variants_to_error,)*
                };
            }
        }
    };
}
