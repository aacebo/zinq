mod input;
mod lazy_parse;
mod r#struct;
mod type_entry;

use std::{cell::RefCell, collections::HashMap};

use input::*;
use lazy_parse::*;
use proc_macro::TokenStream;
use quote::quote;
use type_entry::*;

thread_local! {
    pub(crate) static EXTENDABLE_REGISTRY: RefCell<HashMap<String, TypeEntry>> =
        RefCell::new(HashMap::new());
}

#[proc_macro_attribute]
pub fn extend(input_tokens: TokenStream, item_tokens: TokenStream) -> TokenStream {
    let mut item = syn::parse_macro_input!(item_tokens as syn::Item);

    return match &mut item {
        syn::Item::Struct(struct_item) => r#struct::extend(input_tokens, struct_item).into(),
        _ => quote!(compile_error!("type does not support extensions")).into(),
    };
}
