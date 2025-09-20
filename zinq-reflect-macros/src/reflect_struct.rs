use std::collections::BTreeMap;

use proc_macro::TokenStream;
use quote::quote;

pub fn reflect_struct(input: &syn::DeriveInput, ty: &syn::DataStruct) -> TokenStream {
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
