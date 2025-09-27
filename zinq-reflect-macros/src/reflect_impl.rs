use quote::quote;

use crate::{reflect_generics, reflect_meta};

pub fn build(item: &syn::ItemImpl) -> proc_macro2::TokenStream {
    let impl_for = &item.self_ty;
    let impl_meta = reflect_meta::build(&item.attrs);
    let impl_generics = reflect_generics::build(&item.generics);
    let impl_trait = match &item.trait_ {
        None => None,
        Some((_, path, _)) => Some(quote!(#path)),
    };

    return match &impl_trait {
        None => quote! {
            ::zinq_reflect::Impl::new(&(::zinq_reflect::Path::from(module_path!())), &(::zinq_reflect::type_of!(#impl_for)))
                .meta(&#impl_meta)
                .generics(&#impl_generics)
                .build()
        },
        Some(of) => quote! {
            ::zinq_reflect::Impl::new(&(::zinq_reflect::Path::from(module_path!())), &(::zinq_reflect::type_of!(#impl_for)))
                .meta(&#impl_meta)
                .generics(&#impl_generics)
                .of(&(::zinq_reflect::Path::from(stringify!(#of))))
                .build()
        },
    };
}
