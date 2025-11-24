use proc_macro::TokenStream;
use quote::quote;

use crate::{EXTENDABLE_REGISTRY, TypeEntry};

pub fn extend(input_tokens: TokenStream, item: &mut syn::ItemStruct) -> TokenStream {
    let input = syn::parse_macro_input!(input_tokens as crate::Input);

    for name in input.0.iter() {
        let fields_to_add = match EXTENDABLE_REGISTRY
            .with_borrow(|registry| registry.get(&name.to_string()).cloned())
        {
            None => {
                let message = format!("type '{}' not found", &name);
                return quote!(compile_error(#message)).into();
            }
            Some(entry) => match entry.declare.get() {
                syn::Item::Struct(s) => s.fields.clone(),
                _ => return quote!(compile_error("extend type must be struct")).into(),
            },
        };

        match fields_to_add {
            syn::Fields::Named(named_fields) => match &mut item.fields {
                syn::Fields::Named(fields) => {
                    fields.named.extend(named_fields.named.clone());
                }
                _ => {
                    return quote!(compile_error(
                        "structs can only extend structs with the same field layout"
                    ))
                    .into();
                }
            },
            syn::Fields::Unnamed(unnamed_fields) => match &mut item.fields {
                syn::Fields::Unnamed(fields) => {
                    fields.unnamed.extend(unnamed_fields.unnamed.clone());
                }
                _ => {
                    return quote!(compile_error(
                        "structs can only extend structs with the same field layout"
                    ))
                    .into();
                }
            },
            _ => {}
        };
    }

    EXTENDABLE_REGISTRY.with_borrow_mut(|registry| {
        registry.insert(
            item.ident.to_string(),
            TypeEntry::from(syn::Item::Struct(item.clone())),
        );
    });

    return quote!(#item).into();
}
