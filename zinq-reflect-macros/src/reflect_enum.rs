use quote::quote;

pub fn reflect_enum(input: &syn::DeriveInput, ty: &syn::DataEnum) -> proc_macro2::TokenStream {
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

    return quote! {
        impl ::zinq_reflect::TypeOf for #name {
            fn type_of() -> ::zinq_reflect::Type {
                return ::zinq_reflect::EnumType::new(
                    stringify!(#name),
                    &[#(#variants,)*],
                ).to_type();
            }
        }
    };
}
