use quote::{format_ident, quote};

pub fn render(input: &syn::DeriveInput, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let variants = data
        .variants
        .iter()
        .map(|variant| {
            let variant_ident = &variant.ident;
            let variant_attr = &variant
                .attrs
                .iter()
                .find(|attribute| attribute.path().is_ident("error"));

            let variant_params = match variant_attr {
                None => None,
                Some(attr) => match attr.parse_args::<crate::Params>() {
                    Err(err) => return err.to_compile_error(),
                    Ok(v) => Some(v),
                },
            };

            let variant_fields = match &variant.fields {
                syn::Fields::Unit => vec![],
                syn::Fields::Named(fields) => fields
                    .named
                    .iter()
                    .map(|field| {
                        let field_ident = &field.ident;
                        quote!(#field_ident)
                    })
                    .collect::<Vec<_>>(),
                syn::Fields::Unnamed(fields) => fields
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(i, _)| {
                        let field_ident = format_ident!("p{}", i);
                        quote!(#field_ident)
                    })
                    .collect::<Vec<_>>(),
            };

            let variant_error_name = match &variant_params {
                None => quote!(stringify!(#variant_ident)),
                Some(p) => match &p.name {
                    None => quote!(stringify!(#variant_ident)),
                    Some(v) => quote!(&#v),
                },
            };

            let variant_error_code = match &variant_params {
                None => quote!(None),
                Some(p) => match &p.code {
                    None => quote!(None),
                    Some(v) => quote!(Some(#v)),
                },
            };

            let variant_error_message = match &variant_params {
                None => quote!(None),
                Some(p) => match &p.message {
                    None => quote!(None),
                    Some(v) => quote!(Some(#v)),
                },
            };

            let variant_error = quote! {
                let mut builder = ::zinq_error::Error::new()
                    .with_path(&String::from(module_path!()))
                    .with_name(#variant_error_name)
                    .with_attribute_as("name", &#variant_error_name);

                if let Some(code) = #variant_error_code {
                    builder = builder.with_code(code).with_attribute_as("code", code);
                }

                if let Some(message) = #variant_error_message {
                    builder = builder.with_message(message).with_attribute_as("message", message);
                }

                builder.build()
            };

            if variant_fields.is_empty() {
                return quote!(Self::#variant_ident => {
                    #variant_error
                });
            }

            if variant_fields.len() == 1 {
                return quote!(Self::#variant_ident(v) => {
                    #variant_error
                });
            }

            quote! {
                Self::#variant_ident(#(#variant_fields,)*) => {
                    #variant_error
                }
            }
        })
        .collect::<Vec<_>>();

    return quote! {
        impl ::zinq_error::ToError for #name {
            fn to_error(&self) -> ::zinq_error::Error {
                return match self {
                    #(#variants,)*
                };
            }
        }
    };
}
