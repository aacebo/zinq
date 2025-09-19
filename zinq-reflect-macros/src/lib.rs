use std::collections::BTreeMap;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Reflect)]
pub fn derive_reflect(tokens: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(tokens as syn::DeriveInput);
    let name = input.ident;
    let fields = match &input.data {
        syn::Data::Struct(s) => match &s.fields {
            syn::Fields::Named(field) => {
                let mut fields = BTreeMap::<syn::Ident, syn::Type>::new();

                for f in &field.named {
                    let name = f.ident.as_ref().unwrap().clone();
                    fields.insert(name, f.ty.clone());
                }

                fields
            }
            _ => BTreeMap::<syn::Ident, syn::Type>::new(),
        },
        _ => {
            return syn::Error::new_spanned(&name, "#[derive(Reflect)] only supports structs")
                .to_compile_error()
                .into();
        }
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
