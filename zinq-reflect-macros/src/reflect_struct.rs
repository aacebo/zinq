use quote::quote;

use crate::reflect_visibility;

pub fn reflect_struct(input: &syn::DeriveInput, ty: &syn::DataStruct) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let vis = reflect_visibility(&input.vis);
    let members = match &ty.fields {
        syn::Fields::Named(named_fields) => named_fields
            .named
            .iter()
            .map(|field| {
                let field_name = &field.ident;
                let field_type = &field.ty;
                let field_vis = reflect_visibility(&field.vis);

                quote! {
                    ::zinq_reflect::Member::Field(
                        ::zinq_reflect::Field::new(
                            #field_vis,
                            stringify!(#field_name),
                            &(::zinq_reflect::type_of!(#field_type)),
                        )
                    )
                }
            })
            .collect::<Vec<_>>(),
        syn::Fields::Unnamed(unnamed_fields) => unnamed_fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(i, field)| {
                let field_name = &i.to_string();
                let field_type = &field.ty;
                let field_vis = reflect_visibility(&field.vis);

                quote! {
                    ::zinq_reflect::Member::Field(
                        ::zinq_reflect::Field::new(
                            #field_vis,
                            #field_name,
                            &(::zinq_reflect::type_of!(#field_type)),
                        )
                    )
                }
            })
            .collect::<Vec<_>>(),
        syn::Fields::Unit => vec![],
    };

    return quote! {
        impl ::zinq_reflect::TypeOf for #name {
            fn type_of() -> ::zinq_reflect::Type {
                return ::zinq_reflect::StructType::new(
                    #vis,
                    stringify!(#name),
                    &[#(#members,)*],
                ).to_type();
            }
        }
    };
}
