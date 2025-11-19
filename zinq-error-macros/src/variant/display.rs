use quote::{format_ident, quote};

pub fn render(input: &syn::DeriveInput, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let variants = data
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
                    #(#variants,)*
                };
            }
        }
    };
}
