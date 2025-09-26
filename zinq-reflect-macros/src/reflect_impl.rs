use quote::quote;

pub fn build(item: &syn::ItemImpl) -> proc_macro2::TokenStream {
    let impl_for = &item.self_ty;
    let impl_trait = match &item.trait_ {
        None => None,
        Some((_, path, _)) => Some(quote!(#path)),
    };

    return match &impl_trait {
        None => quote! {
            ::zinq_reflect::Impl::new(&(::zinq_reflect::Path::from(module_path!())), &(::zinq_reflect::type_of!(#impl_for)))
                .build()
        },
        Some(of) => quote! {
            ::zinq_reflect::Impl::new(&(::zinq_reflect::Path::from(module_path!())), &(::zinq_reflect::type_of!(#impl_for)))
                .of(&(::zinq_reflect::Path::from(stringify!(#of))))
                .build()
        },
    };
}
