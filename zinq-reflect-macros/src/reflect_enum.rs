use quote::quote;

use crate::{reflect_field, reflect_visibility};

pub fn derive(input: &syn::DeriveInput, ty: &syn::DataEnum) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let vis = reflect_visibility::derive(&input.vis);
    let variants = ty
        .variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;

            match &variant.fields {
                syn::Fields::Unit => quote! {
                    ::zinq_reflect::Variant::new(stringify!(#variant_name)).build()
                },
                syn::Fields::Named(named_fields) => {
                    let fields = named_fields
                        .named
                        .iter()
                        .enumerate()
                        .map(|(i, field)| reflect_field::derive(field, i, true))
                        .collect::<Vec<_>>();

                    quote! {
                        ::zinq_reflect::Variant::new(stringify!(#variant_name))
                            .fields(
                                ::zinq_reflect::Fields::new()
                                    .layout(::zinq_reflect::Layout::Key)
                                    .fields(&[#(#fields,)*])
                                    .build()
                                    .as_ref()
                            )
                            .build()
                    }
                }
                syn::Fields::Unnamed(unnamed_fields) => {
                    let fields = unnamed_fields
                        .unnamed
                        .iter()
                        .enumerate()
                        .map(|(i, field)| reflect_field::derive(field, i, false))
                        .collect::<Vec<_>>();

                    quote! {
                        ::zinq_reflect::Variant::new(stringify!(#variant_name))
                            .fields(
                                ::zinq_reflect::Fields::new()
                                    .layout(::zinq_reflect::Layout::Index)
                                    .fields(&[#(#fields,)*])
                                    .build()
                                    .as_ref()
                            )
                            .build()
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    return quote! {
        impl ::zinq_reflect::TypeOf for #name {
            fn type_of() -> ::zinq_reflect::Type {
                return ::zinq_reflect::EnumType::new(&(::zinq_reflect::Path::from(module_path!())), stringify!(#name))
                    .visibility(#vis)
                    .variants(&[#(#variants,)*])
                    .build()
                    .to_type();
            }
        }
    };
}
