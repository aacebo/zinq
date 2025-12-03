use quote::quote;

use crate::{reflect_meta, reflect_visibility};

pub fn build(field: &syn::Field, index: usize, is_named: bool) -> proc_macro2::TokenStream {
    let field_ident = &field.ident;
    let field_name = if is_named {
        quote!(::reflectx::FieldName::from(stringify!(#field_ident)))
    } else {
        quote!(::reflectx::FieldName::from(#index))
    };

    let field_type = &field.ty;
    let field_vis = reflect_visibility::build(&field.vis);
    let field_meta = reflect_meta::build(&field.attrs);

    return quote! {
        ::reflectx::Field::new()
            .with_name(&#field_name)
            .with_type(&(::reflectx::type_of!(#field_type)))
            .with_visibility(#field_vis)
            .with_meta(&#field_meta)
            .build()
    };
}
