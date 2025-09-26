use quote::quote;

use crate::parse;

pub fn ty(attributes: &[syn::Attribute]) -> proc_macro2::TokenStream {
    let mut pairs = vec![];

    for attr in attributes.iter().filter(|a| a.path().is_ident("reflect")) {
        let _ = attr.parse_nested_meta(|meta| {
            pairs.push(parse::meta_data_item(meta));
            Ok(())
        });
    }

    return quote! {
        ::zinq_reflect::MetaData::from([#(#pairs,)*])
    };
}
