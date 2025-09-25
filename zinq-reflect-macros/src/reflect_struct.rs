use quote::quote;

use crate::{parse, reflect_field, reflect_visibility};

pub fn derive(input: &syn::DeriveInput, ty: &syn::DataStruct) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let vis = reflect_visibility::derive(&input.vis);
    let layout = match &ty.fields {
        syn::Fields::Named(_) => quote!(::zinq_reflect::Layout::Key),
        syn::Fields::Unnamed(_) => quote!(::zinq_reflect::Layout::Index),
        syn::Fields::Unit => quote!(::zinq_reflect::Layout::Unit),
    };

    let mut pairs = vec![];
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

    for attr in input.attrs.iter().filter(|a| a.path().is_ident("reflect")) {
        let _ = attr.parse_nested_meta(|meta| {
            pairs.push(parse::meta_data_item(meta));
            Ok(())
        });
    }

    return quote! {
        impl ::zinq_reflect::TypeOf for #name {
            fn type_of() -> ::zinq_reflect::Type {
                return ::zinq_reflect::StructType::new(&(::zinq_reflect::Module::from(module_path!())), stringify!(#name))
                    .visibility(#vis)
                    .meta(&(::zinq_reflect::MetaData::from([#(#pairs,)*])))
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
