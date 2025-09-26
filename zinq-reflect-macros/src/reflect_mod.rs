use quote::quote;

use crate::{reflect_meta, reflect_visibility};

pub fn attr(meta: proc_macro2::TokenStream, item: &mut syn::ItemMod) -> proc_macro2::TokenStream {
    if item.content.is_some() {
        let value = ty(meta, item);
        item.content
            .as_mut()
            .unwrap()
            .1
            .push(syn::Item::Verbatim(quote! {
                pub fn type_of() -> ::zinq_reflect::Type {
                    return #value;
                }
            }));
    }

    return quote!(#item);
}

pub fn ty(meta: proc_macro2::TokenStream, item: &mut syn::ItemMod) -> proc_macro2::TokenStream {
    let vis = reflect_visibility::derive(&item.vis);
    let inner_meta = reflect_meta::ty(&item.attrs);
    let mut types = vec![];

    if let Some((_, items)) = &mut item.content {
        for item in items.iter_mut() {
            match super::reflect_ty(item) {
                None => continue,
                Some(v) => types.push(v),
            };
        }
    }

    return quote! {
        ::zinq_reflect::ModType::new(&(::zinq_reflect::Path::from(module_path!())))
            .meta(&#meta.merge(&#inner_meta))
            .visibility(#vis)
            .types(&[#(#types,)*])
            .build()
            .to_type()
    };
}
