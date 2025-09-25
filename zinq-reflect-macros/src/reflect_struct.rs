use quote::quote;

use crate::{reflect_field, reflect_visibility};

pub fn derive(input: &syn::DeriveInput, ty: &syn::DataStruct) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let vis = reflect_visibility::derive(&input.vis);
    let layout = match &ty.fields {
        syn::Fields::Named(_) => quote!(::zinq_reflect::Layout::Key),
        syn::Fields::Unnamed(_) => quote!(::zinq_reflect::Layout::Index),
        syn::Fields::Unit => quote!(::zinq_reflect::Layout::Unit),
    };

    let fields = match &ty.fields {
        syn::Fields::Named(named_fields) => named_fields
            .named
            .iter()
            .enumerate()
            .map(|(i, field)| reflect_field::derive(field, i, true))
            .collect::<Vec<_>>(),
        syn::Fields::Unnamed(unnamed_fields) => unnamed_fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(i, field)| reflect_field::derive(field, i, false))
            .collect::<Vec<_>>(),
        syn::Fields::Unit => vec![],
    };

    return quote! {
        impl ::zinq_reflect::TypeOf for #name {
            fn type_of() -> ::zinq_reflect::Type {
                return ::zinq_reflect::StructType::new(&(::zinq_reflect::Module::from(module_path!())), stringify!(#name))
                    .visibility(#vis)
                    .fields(
                        ::zinq_reflect::Fields::new()
                            .layout(#layout)
                            .fields(&[#(#fields,)*])
                            .build()
                            .as_ref()
                    )
                    .build()
                    .to_type();
            }
        }
    };
}
