use quote::quote;

pub fn build(generics: &syn::Generics) -> proc_macro2::TokenStream {
    let mut params = vec![];

    for param in &generics.params {
        params.push(match param {
            syn::GenericParam::Lifetime(v) => build_lifetime(v),
            syn::GenericParam::Type(v) => build_type(v),
            syn::GenericParam::Const(v) => build_const(v),
        });
    }

    if params.is_empty() {
        return quote!(::zinq_reflect::Generics::new());
    }

    return quote! {
        ::zinq_reflect::Generics::from([#(#params.to_generic(),)*])
    };
}

pub fn build_lifetime(param: &syn::LifetimeParam) -> proc_macro2::TokenStream {
    let name = &param.lifetime.ident;
    let mut bounds = vec![];

    for lifetime in &param.bounds {
        let lifetime_name = &lifetime.ident;

        bounds.push(quote! {
            ::zinq_reflect::LifetimeBound::new(stringify!(#lifetime_name))
        });
    }

    return quote! {
        ::zinq_reflect::LifetimeParam::new(
            stringify!(#name),
            &[#(#bounds.to_bound(),)*],
        )
    };
}

pub fn build_type(param: &syn::TypeParam) -> proc_macro2::TokenStream {
    let name = &param.ident;
    let mut bounds = vec![];

    for ty in &param.bounds {
        bounds.push(build_bound(ty));
    }

    let tokens = quote! {
        ::zinq_reflect::TypeParam::new(stringify!(#name))
            .bounds(&[#(#bounds.to_bound(),)*])
    };

    return match &param.default {
        None => quote!(#tokens.build()),
        Some(default) => quote!(#tokens.default(&(::zinq_reflect::type_of!(#default))).build()),
    };
}

pub fn build_const(param: &syn::ConstParam) -> proc_macro2::TokenStream {
    let name = &param.ident;
    let ty = &param.ty;
    let tokens = quote! {
        ::zinq_reflect::ConstParam::new(
            stringify!(#name),
            &(::zinq_reflect::type_of!(#ty)),
        )
    };

    return match &param.default {
        None => tokens,
        Some(default) => quote!(#tokens.with_default(#default)),
    };
}

pub fn build_bound(bound: &syn::TypeParamBound) -> proc_macro2::TokenStream {
    return match bound {
        syn::TypeParamBound::Lifetime(v) => build_lifetime_bound(v),
        syn::TypeParamBound::Trait(v) => build_trait_bound(v),
        syn::TypeParamBound::Verbatim(v) => v.clone(),
        _ => quote!(),
    };
}

pub fn build_lifetime_bound(bound: &syn::Lifetime) -> proc_macro2::TokenStream {
    let name = &bound.ident;

    return quote! {
        ::zinq_reflect::LifetimeBound::new(stringify!(#name))
    };
}

pub fn build_trait_bound(bound: &syn::TraitBound) -> proc_macro2::TokenStream {
    let path = &bound.path;
    let modifier = match &bound.modifier {
        syn::TraitBoundModifier::None => quote!(::zinq_reflect::TraitBoundModifier::None),
        syn::TraitBoundModifier::Maybe(_) => quote!(::zinq_reflect::TraitBoundModifier::Maybe),
    };

    return quote! {
        ::zinq_reflect::TraitBound::new(
            &(::zinq_reflect::Path::from(#path)),
            #modifier,
        )
    };
}
