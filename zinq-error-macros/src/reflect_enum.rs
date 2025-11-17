use quote::{format_ident, quote};

pub fn derive(input: &syn::DeriveInput, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let variants_write = data
        .variants
        .iter()
        .map(|variant| {
            let variant_ident = &variant.ident;
            let variant_attr = &variant
                .attrs
                .iter()
                .map(|attribute| {
                    if !attribute.path().is_ident("error") {
                        return quote!();
                    }

                    let message: syn::LitStr =
                        attribute.parse_args().expect("expected string literal");
                    quote!(#message)
                })
                .last();

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

            if let Some(tokens) = variant_attr {
                return quote! {
                    Self::#variant_ident => write!(f, "{}", #tokens)
                };
            }

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
                return ::zinq_error::Error::new()
                    .with_name(stringify!(#name))
                    .with_message(&self.to_string())
                    .build();
            }
        }
    };
}
