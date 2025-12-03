use quote::{format_ident, quote};

use crate::{reflect_field, reflect_generics, reflect_meta, reflect_visibility};

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
                    Self::#variant_ident => ::reflectx::Value::Null
                };
            }

            if variant_fields.len() == 1 {
                return quote! {
                    Self::#variant_ident(v) => ::reflectx::value_of!(v)
                };
            }

            quote! {
                Self::#variant_ident(#(#variant_fields,)*) => {
                    ::reflectx::value_of!((#(#variant_fields,)*))
                }
            }
        })
        .collect::<Vec<_>>();

    return quote! {
        impl ::reflectx::TypeOf for #name {
            fn type_of() -> ::reflectx::Type {
                return #ty;
            }
        }

        impl ::reflectx::ToType for #name {
            fn to_type(&self) -> ::reflectx::Type {
                return #ty;
            }
        }

        impl ::reflectx::ToValue for #name {
            fn to_value(self) -> ::reflectx::Value {
                return match self {
                    #(#variants,)*
                };
            }
        }

        impl ::reflectx::Dyn for #name { }

        impl ::reflectx::AsValue for #name {
            fn as_value(&self) -> ::reflectx::Value {
                return match self {
                    #(#variants,)*
                };
            }
        }
    };
}

pub fn attr(item: &syn::ItemEnum) -> proc_macro2::TokenStream {
    let name = &item.ident;
    let ty = build(item);
    let variants = item
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
                    Self::#variant_ident => ::reflectx::Value::Null
                };
            }

            if variant_fields.len() == 1 {
                return quote! {
                    Self::#variant_ident(v) => ::reflectx::value_of!(v)
                };
            }

            quote! {
                Self::#variant_ident(#(#variant_fields,)*) => {
                    ::reflectx::value_of!((#(#variant_fields,)*))
                }
            }
        })
        .collect::<Vec<_>>();

    return quote! {
        impl ::reflectx::TypeOf for #name {
            fn type_of() -> ::reflectx::Type {
                return #ty;
            }
        }

        impl ::reflectx::ToType for #name {
            fn to_type(&self) -> ::reflectx::Type {
                return #ty;
            }
        }

        impl ::reflectx::ToValue for #name {
            fn to_value(self) -> ::reflectx::Value {
                return match self {
                    #(#variants,)*
                };
            }
        }

        impl ::reflectx::Dyn for #name { }

        impl ::reflectx::AsValue for #name {
            fn as_value(&self) -> ::reflectx::Value {
                return match self {
                    #(#variants,)*
                };
            }
        }
    };
}

pub fn build(item: &syn::ItemEnum) -> proc_macro2::TokenStream {
    let name = &item.ident;
    let vis = reflect_visibility::build(&item.vis);
    let meta = reflect_meta::build(&item.attrs);
    let generics = reflect_generics::build(&item.generics);
    let variants = item
        .variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;
            let variant_meta = reflect_meta::build(&variant.attrs);

            match &variant.fields {
                syn::Fields::Unit => quote! {
                    ::reflectx::Variant::new()
                        .with_name(stringify!(#variant_name))
                        .build()
                },
                syn::Fields::Named(named_fields) => {
                    let fields = named_fields
                        .named
                        .iter()
                        .enumerate()
                        .map(|(i, field)| reflect_field::build(field, i, true))
                        .collect::<Vec<_>>();

                    quote! {
                        ::reflectx::Variant::new()
                            .with_name(stringify!(#variant_name))
                            .with_meta(&#variant_meta)
                            .with_fields(
                                ::reflectx::Fields::new()
                                    .with_layout(::reflectx::Layout::Key)
                                    .with_fields(&[#(#fields,)*])
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
                        ::reflectx::Variant::new()
                            .with_name(stringify!(#variant_name))
                            .with_meta(&#variant_meta)
                            .with_fields(
                                ::reflectx::Fields::new()
                                    .with_layout(::reflectx::Layout::Index)
                                    .with_fields(&[#(#fields,)*])
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
        ::reflectx::EnumType::new()
            .with_path(&(::reflectx::Path::from(module_path!())))
            .with_name(stringify!(#name))
            .with_meta(&#meta)
            .with_generics(&#generics)
            .with_visibility(#vis)
            .with_variants(&[#(#variants,)*])
            .build()
            .to_type()
    };
}
