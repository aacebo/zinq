use quote::quote;

use crate::reflect_visibility;

pub fn reflect_enum(input: &syn::DeriveInput, ty: &syn::DataEnum) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let vis = reflect_visibility(&input.vis);
    let variants = ty
        .variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;

            match &variant.fields {
                syn::Fields::Unit => quote! {
                    ::zinq_reflect::Variant::Unit(
                        ::zinq_reflect::UnitVariant::new(stringify!(#variant_name))
                    )
                },
                syn::Fields::Named(named_fields) => {
                    let fields = named_fields
                        .named
                        .iter()
                        .map(|field| {
                            let field_name = &field.ident;
                            let field_type = &field.ty;
                            let field_vis = reflect_visibility(&field.vis);

                            quote! {
                                ::zinq_reflect::Field::new(
                                    #field_vis,
                                    stringify!(#field_name),
                                    &(::zinq_reflect::type_of!(#field_type)),
                                )
                            }
                        })
                        .collect::<Vec<_>>();

                    quote! {
                        ::zinq_reflect::Variant::Struct(
                            ::zinq_reflect::StructVariant::new(
                                stringify!(#variant_name),
                                &[#(#fields,)*],
                            )
                        )
                    }
                }
                syn::Fields::Unnamed(unnamed_fields) => {
                    let types = unnamed_fields
                        .unnamed
                        .iter()
                        .map(|field| {
                            let field_type = &field.ty;

                            quote! {
                                ::zinq_reflect::type_of!(#field_type)
                            }
                        })
                        .collect::<Vec<_>>();

                    quote! {
                        ::zinq_reflect::Variant::Tuple(
                            ::zinq_reflect::TupleVariant::new(
                                stringify!(#variant_name),
                                &[#(#types,)*],
                            )
                        )
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    return quote! {
        impl ::zinq_reflect::TypeOf for #name {
            fn type_of() -> ::zinq_reflect::Type {
                return ::zinq_reflect::EnumType::new(
                    #vis,
                    stringify!(#name),
                    &[#(#variants,)*],
                ).to_type();
            }
        }
    };
}
