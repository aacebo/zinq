use quote::quote;

use crate::{reflect_field, reflect_meta, reflect_visibility};

pub fn derive(input: &syn::DeriveInput, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let ty = build(&syn::ItemEnum {
        attrs: input.attrs.clone(),
        variants: data.variants.clone(),
        generics: input.generics.clone(),
        ident: input.ident.clone(),
        brace_token: data.brace_token.clone(),
        enum_token: data.enum_token.clone(),
        vis: input.vis.clone(),
    });

    return quote! {
        impl ::zinq_reflect::TypeOf for #name {
            fn type_of() -> ::zinq_reflect::Type {
                return #ty;
            }
        }
    };
}

pub fn attr(item: &syn::ItemEnum) -> proc_macro2::TokenStream {
    let name = &item.ident;
    let ty = build(item);

    return quote! {
        impl ::zinq_reflect::TypeOf for #name {
            fn type_of() -> ::zinq_reflect::Type {
                return #ty;
            }
        }
    };
}

pub fn build(item: &syn::ItemEnum) -> proc_macro2::TokenStream {
    let name = &item.ident;
    let vis = reflect_visibility::build(&item.vis);
    let meta = reflect_meta::build(&item.attrs);
    let variants = item
        .variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;
            let variant_meta = reflect_meta::build(&variant.attrs);

            match &variant.fields {
                syn::Fields::Unit => quote! {
                    ::zinq_reflect::Variant::new(stringify!(#variant_name)).build()
                },
                syn::Fields::Named(named_fields) => {
                    let fields = named_fields
                        .named
                        .iter()
                        .enumerate()
                        .map(|(i, field)| reflect_field::build(field, i, true))
                        .collect::<Vec<_>>();

                    quote! {
                        ::zinq_reflect::Variant::new(stringify!(#variant_name))
                            .meta(&#variant_meta)
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
                        .map(|(i, field)| reflect_field::build(field, i, false))
                        .collect::<Vec<_>>();

                    quote! {
                        ::zinq_reflect::Variant::new(stringify!(#variant_name))
                            .meta(&#variant_meta)
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
        ::zinq_reflect::EnumType::new(&(::zinq_reflect::Path::from(module_path!())), stringify!(#name))
            .meta(&#meta)
            .visibility(#vis)
            .variants(&[#(#variants,)*])
            .build()
            .to_type()
    };
}
