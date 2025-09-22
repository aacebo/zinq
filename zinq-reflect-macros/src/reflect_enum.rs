use quote::quote;

use crate::reflect_visibility;

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
                        .map(|field| {
                            let field_name = &field.ident;
                            let field_type = &field.ty;
                            let field_vis = reflect_visibility::derive(&field.vis);

                            quote! {
                                ::zinq_reflect::Field::new(
                                    &(::zinq_reflect::FieldName::from(stringify!(#field_name))),
                                    &(::zinq_reflect::type_of!(#field_type)),
                                )
                                .visibility(#field_vis)
                                .build()
                            }
                        })
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
                        .map(|(i, field)| {
                            let field_type = &field.ty;
                            let field_vis = reflect_visibility::derive(&field.vis);

                            quote! {
                                ::zinq_reflect::Field::new(
                                    &(::zinq_reflect::FieldName::from(#i)),
                                    &(::zinq_reflect::type_of!(#field_type)),
                                )
                                .visibility(#field_vis)
                                .build()
                            }
                        })
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

    // let variant_values = ty
    //     .variants
    //     .iter()
    //     .map(|variant| {
    //         let variant_name = &variant.ident;

    //         match &variant.fields {
    //             syn::Fields::Named(named_fields) => {
    //                 let fields = named_fields
    //                     .named
    //                     .iter()
    //                     .map(|field| {
    //                         let field_name = &field.ident;
    //                         quote!(stringify!(#field_name))
    //                     })
    //                     .collect::<Vec<_>>();

    //                 quote! {
    //                     Self::#variant_name { #(#fields,)* } => {

    //                     }
    //                 }
    //             }
    //             syn::Fields::Unnamed(unnamed_fields) => {
    //                 let fields = unnamed_fields
    //                     .unnamed
    //                     .iter()
    //                     .enumerate()
    //                     .map(|(i, _)| {
    //                         let field_name = &i.to_string();
    //                         quote!(#field_name)
    //                     })
    //                     .collect::<Vec<_>>();

    //                 quote! {
    //                     Self::#variant_name (#(#fields,)*) => {}
    //                 }
    //             }
    //             syn::Fields::Unit => quote! {
    //                 Self::#variant_name => {}
    //             },
    //         }
    //     })
    //     .collect::<Vec<_>>();

    return quote! {
        impl ::zinq_reflect::TypeOf for #name {
            fn type_of() -> ::zinq_reflect::Type {
                return ::zinq_reflect::EnumType::new(&(::zinq_reflect::Module::from(module_path!())), stringify!(#name))
                    .visibility(#vis)
                    .variants(&[#(#variants,)*])
                    .build()
                    .to_type();
            }
        }

        // impl ::zinq_reflect::Reflect for #name {
        //     fn reflect(self) -> ::zinq_reflect::Value {
        //         return match &self {
        //             #(#variant_values,)*
        //         };
        //     }
        // }
    };
}
