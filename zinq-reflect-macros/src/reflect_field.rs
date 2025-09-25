use quote::quote;

use crate::{parse, reflect_visibility};

pub fn derive(field: &syn::Field, index: usize, is_named: bool) -> proc_macro2::TokenStream {
    let field_ident = &field.ident;
    let field_name = if is_named {
        quote!(::zinq_reflect::FieldName::from(stringify!(#field_ident)))
    } else {
        quote!(::zinq_reflect::FieldName::from(#index))
    };

    let field_type = &field.ty;
    let field_vis = reflect_visibility::derive(&field.vis);
    let mut pairs = vec![];

    for attr in field.attrs.iter().filter(|a| a.path().is_ident("reflect")) {
        let _ = attr.parse_nested_meta(|meta| {
            pairs.push(parse::meta_data_item(meta));
            Ok(())
        });
    }

    return quote! {
        ::zinq_reflect::Field::new(
            &#field_name,
            &(::zinq_reflect::type_of!(#field_type)),
        )
        .visibility(#field_vis)
        .meta(&(::zinq_reflect::MetaData::from([#(#pairs)*])))
        .build()
    };
}
