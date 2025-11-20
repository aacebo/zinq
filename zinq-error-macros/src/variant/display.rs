use quote::{format_ident, quote};

pub fn render(input: &syn::DeriveInput, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let variants = data
        .variants
        .iter()
        .map(|variant| {
            let variant_ident = &variant.ident;
            let (is_empty, variant_fields_inner, variant_fields_outer) = match &variant.fields {
                syn::Fields::Unit => (true, quote!(), quote!()),
                syn::Fields::Named(fields) => {
                    let inner = fields
                        .named
                        .iter()
                        .map(|field| {
                            let field_ident = &field.ident;
                            quote!(#field_ident)
                        })
                        .collect::<Vec<_>>();

                    (
                        inner.is_empty(),
                        quote!(#(#inner = #inner, )*),
                        quote!({ #(#inner, )* }),
                    )
                }
                syn::Fields::Unnamed(fields) => {
                    let inner = fields
                        .unnamed
                        .iter()
                        .enumerate()
                        .map(|(i, _)| {
                            let field_ident = format_ident!("p{}", i);
                            quote!(#field_ident)
                        })
                        .collect::<Vec<_>>();

                    (
                        inner.is_empty(),
                        quote!(#(#inner, )*),
                        quote!((#(#inner, )*)),
                    )
                }
            };

            if is_empty {
                return quote! {
                    Self::#variant_ident #variant_fields_outer => {
                        write!(f, "{}", stringify!(#variant_ident))
                    }
                };
            }

            quote! {
                Self::#variant_ident #variant_fields_outer => {
                    write!(f, "{}", #variant_fields_inner)
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
