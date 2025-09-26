use quote::quote;

use crate::{reflect_field, reflect_meta, reflect_visibility};

pub fn derive(input: &syn::DeriveInput, data: &syn::DataStruct) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let ty = ty(&syn::ItemStruct {
        attrs: input.attrs.clone(),
        fields: data.fields.clone(),
        generics: input.generics.clone(),
        ident: input.ident.clone(),
        semi_token: data.semi_token.clone(),
        struct_token: data.struct_token.clone(),
        vis: input.vis.clone(),
    });

    return quote! {
        impl ::zinq_reflect::TypeOf for #name {
            fn type_of() -> ::zinq_reflect::Type {
                return #ty;
            }
        }
    };
}

pub fn attr(item: &syn::ItemStruct) -> proc_macro2::TokenStream {
    let name = &item.ident;
    let ty = ty(item);

    return quote! {
        impl ::zinq_reflect::TypeOf for #name {
            fn type_of() -> ::zinq_reflect::Type {
                return #ty;
            }
        }
    };
}

pub fn ty(item: &syn::ItemStruct) -> proc_macro2::TokenStream {
    let name = &item.ident;
    let vis = reflect_visibility::derive(&item.vis);
    let layout = match &item.fields {
        syn::Fields::Named(_) => quote!(::zinq_reflect::Layout::Key),
        syn::Fields::Unnamed(_) => quote!(::zinq_reflect::Layout::Index),
        syn::Fields::Unit => quote!(::zinq_reflect::Layout::Unit),
    };

    let meta = reflect_meta::ty(&item.attrs);
    let fields = match &item.fields {
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
        ::zinq_reflect::StructType::new(&(::zinq_reflect::Path::from(module_path!())), stringify!(#name))
            .visibility(#vis)
            .meta(&#meta)
            .fields(
                ::zinq_reflect::Fields::new()
                    .layout(#layout)
                    .fields(&[#(#fields,)*])
                    .build()
                    .as_ref()
            )
            .build()
            .to_type()
    };
}
