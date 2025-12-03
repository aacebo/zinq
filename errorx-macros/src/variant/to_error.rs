use quote::{format_ident, quote};

pub fn render(input: &syn::DeriveInput, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let variants = data
        .variants
        .iter()
        .map(|variant| {
            let span = proc_macro2::Span::mixed_site();
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

            let (is_empty, variant_fields_inner, variant_fields_outer) = match &variant.fields {
                syn::Fields::Unit => (true, quote!(), quote!()),
                syn::Fields::Named(fields) => {
                    let inner = fields
                        .named
                        .iter()
                        .map(|field| {
                            let field_ident = &field.ident;
                            quote!(#field_ident)
                        })
                        .collect::<Vec<_>>();

                    (
                        inner.is_empty(),
                        quote!(#(#inner = #inner, )*),
                        quote!({ #(#inner, )* }),
                    )
                }
                syn::Fields::Unnamed(fields) => {
                    let inner = fields
                        .unnamed
                        .iter()
                        .enumerate()
                        .map(|(i, _)| {
                            let field_ident = format_ident!("p{}", i);
                            quote!(#field_ident)
                        })
                        .collect::<Vec<_>>();

                    (
                        inner.is_empty(),
                        quote!(#(#inner, )*),
                        quote!((#(#inner, )*)),
                    )
                }
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
                None => quote!(Option::<String>::None),
                Some(p) => match &p.message {
                    None => quote!(Option::<String>::None),
                    Some(v) => {
                        let literal = syn::LitStr::new(v, span);

                        if is_empty {
                            quote!(Some(format!(#literal)));
                        }

                        quote!(Some(format!(#literal, #variant_fields_inner)))
                    }
                },
            };

            let variant_error = quote! {
                let mut builder = ::errorx::Error::new()
                    .with_path(&String::from(module_path!()))
                    .with_name(#variant_error_name);

                if let Some(code) = #variant_error_code {
                    builder = builder.with_code(code);
                }

                if let Some(message) = #variant_error_message {
                    builder = builder.with_message_string(message.clone());
                }

                builder.build()
            };

            quote! {
                Self::#variant_ident #variant_fields_outer => {
                    #variant_error
                }
            }
        })
        .collect::<Vec<_>>();

    return quote! {
        impl ::errorx::ToError for #name {
            fn to_error(&self) -> ::errorx::Error {
                return match self {
                    #(#variants,)*
                };
            }
        }
    };
}
