use quote::{ToTokens, quote};

pub fn derive(vis: &syn::Visibility) -> proc_macro2::TokenStream {
    return match vis {
        syn::Visibility::Inherited => quote!(::zinq_reflect::Visibility::Private),
        syn::Visibility::Public(_) => quote! {
            ::zinq_reflect::Visibility::Public(
                ::zinq_reflect::Public::Full
            )
        },
        syn::Visibility::Restricted(v) => {
            let path = v.path.to_token_stream().to_string();

            match path.as_str() {
                "self" => quote! {
                    ::zinq_reflect::Visibility::Public(
                        ::zinq_reflect::Public::Type
                    )
                },
                "crate" => quote! {
                    ::zinq_reflect::Visiblity::Public(
                        ::zinq_reflect::Public::Crate
                    )
                },
                "super" => quote! {
                    ::zinq_reflect::Visiblity::Public(
                        ::zinq_reflect::Public::Super
                    )
                },
                other => quote! {
                    ::zinq_reflect::Visiblity::Public(
                        ::zinq_reflect::Public::Mod(#other.to_string())
                    )
                },
            }
        }
    };
}
