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
    let mut pairs = Vec::<proc_macro2::TokenStream>::new();

    for attr in field.attrs.iter().filter(|a| a.path().is_ident("reflect")) {
        if let Err(e) = parse_reflect_kv(attr, &mut pairs) {
            return e.to_compile_error();
        }
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

fn parse_reflect_kv(
    attr: &syn::Attribute,
    out: &mut Vec<proc_macro2::TokenStream>,
) -> syn::Result<()> {
    attr.parse_nested_meta(|meta| {
        let key_ident = meta.path.get_ident().ok_or_else(|| {
            meta.error("reflect keys must be identifiers (e.g., key = \"value\")")
        })?;

        let key_str = key_ident.to_string();

        // If there is no "= …", treat as `key = "true"`
        if meta.input.is_empty() {
            let key_lit = syn::LitStr::new(&key_str, key_ident.span());
            out.push(quote! { (#key_lit, "true") });
            return Ok(());
        }

        // Expect `= "value"`
        meta.input.parse::<syn::Token![=]>()?;

        let val: syn::LitStr = meta.input.parse().map_err(|_| {
            meta.error("reflect values must be string literals, e.g., key = \"value\"")
        })?;

        let key_lit = syn::LitStr::new(&key_str, key_ident.span());
        out.push(quote! { (#key_lit.to_string(), ::zinq_reflect::value_of!(#val)) });
        Ok(())
    })
}
