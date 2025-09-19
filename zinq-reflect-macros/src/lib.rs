use std::collections::BTreeMap;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Reflect)]
pub fn derive_reflect(tokens: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(tokens as syn::DeriveInput);

    return match &input.data {
        syn::Data::Struct(ty) => derive_reflect_struct(&input, ty),
        syn::Data::Enum(ty) => derive_reflect_enum(&input, ty),
        _ => panic!("unsupported Reflect type '{}'", &input.ident),
    };
}

fn derive_reflect_struct(input: &syn::DeriveInput, ty: &syn::DataStruct) -> TokenStream {
    let name = &input.ident;
    let fields = match &ty.fields {
        syn::Fields::Named(field) => {
            let mut fields = BTreeMap::<syn::Ident, syn::Type>::new();

            for f in &field.named {
                let name = f.ident.as_ref().unwrap().clone();
                fields.insert(name, f.ty.clone());
            }

            fields
        }
        _ => BTreeMap::<syn::Ident, syn::Type>::new(),
    };

    let members = fields
        .iter()
        .map(|(name, ty)| {
            quote! {
                ::zinq_reflect::Member::Field(::zinq_reflect::Field::new(
                    zinq_reflect::Visibility::Public,
                    stringify!(#name),
                    &(::zinq_reflect::type_of!(#ty)),
                ))
            }
        })
        .collect::<Vec<_>>();

    return TokenStream::from(quote! {
        impl ::zinq_reflect::TypeOf for #name {
            fn type_of() -> ::zinq_reflect::Type {
                return ::zinq_reflect::StructType::new(
                    stringify!(#name),
                    &[#(#members,)*],
                ).to_type();
            }
        }
    });
}

fn derive_reflect_enum(input: &syn::DeriveInput, ty: &syn::DataEnum) -> TokenStream {
    let name = &input.ident;
    let variants = ty
        .variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;

            match &variant.fields {
                syn::Fields::Unit => quote! {
                    ::zinq_reflect::Variant::Unit(
                        ::zinq_reflect::UnitVariant::new(stringify!(#variant_name)),
                    )
                },
                syn::Fields::Named(field) => {
                    let fields = field
                        .named
                        .iter()
                        .map(|f| {
                            let field_name = &f.ident;
                            let ty = &f.ty;

                            quote! {
                                ::zinq_reflect::Field::new(
                                    zinq_reflect::Visibility::Public,
                                    stringify!(#field_name),
                                    &(::zinq_reflect::type_of!(#ty)),
                                )
                            }
                        })
                        .collect::<Vec<_>>();

                    quote! {
                        ::zinq_reflect::Variant::Struct(
                            ::zinq_reflect::StructVariant::new(
                                stringify!(#variant_name),
                                &[#(#fields,)*],
                            ),
                        ),
                    }
                }
                syn::Fields::Unnamed(field) => {
                    let types = field
                        .unnamed
                        .iter()
                        .map(|f| {
                            let ty = &f.ty;

                            quote! {
                                ::zinq_reflect::type_of!(#ty)
                            }
                        })
                        .collect::<Vec<_>>();

                    quote! {
                        ::zinq_reflect::Variant::Tuple(
                            ::zinq_reflect::TupleVariant::new(
                                stringify!(#variant_name),
                                &[#(#types,)*],
                            ),
                        ),
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    return TokenStream::from(quote! {
        impl ::zinq_reflect::TypeOf for #name {
            fn type_of() -> ::zinq_reflect::Type {
                return ::zinq_reflect::EnumType::new(
                    stringify!(#name),
                    &[#(#variants,)*],
                ).to_type();
            }
        }
    });
}
