use quote::quote;

use crate::reflect_visibility;

pub fn derive(field: &syn::Field, index: usize, is_named: bool) -> proc_macro2::TokenStream {
    let field_ident = &field.ident;
    let field_name = if is_named {
        quote!(::zinq_reflect::FieldName::from(stringify!(#field_ident)))
    } else {
        quote!(::zinq_reflect::FieldName::from(#index))
    };

    let field_type = &field.ty;
    let field_vis = reflect_visibility::derive(&field.vis);

    // for field in &ty.fields {
    //     for attr in field.attrs.iter().filter(|a| a.path().is_ident("field")) {
    //         let _ = attr.parse_nested_meta(|meta| {
    //             if meta.path.is_ident("") {

    //             }

    //             Err(meta.error("unsupported #[field(...)] key"))
    //         });
    //     }
    // }

    return quote! {
        ::zinq_reflect::Field::new(
            &#field_name,
            &(::zinq_reflect::type_of!(#field_type)),
        )
        .visibility(#field_vis)
        .build()
    };
}
