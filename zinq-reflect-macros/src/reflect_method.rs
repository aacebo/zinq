use quote::quote;

use crate::reflect_visibility;

pub fn attr(item: &syn::ItemFn) -> proc_macro2::TokenStream {
    let _vis = reflect_visibility::derive(&item.vis);

    return quote!(#item);
}
