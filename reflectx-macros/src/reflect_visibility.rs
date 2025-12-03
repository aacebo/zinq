use quote::{ToTokens, quote};

pub fn build(vis: &syn::Visibility) -> proc_macro2::TokenStream {
    return match vis {
        syn::Visibility::Inherited => quote!(::reflectx::Visibility::Private),
        syn::Visibility::Public(_) => quote! {
            ::reflectx::Visibility::Public(
                ::reflectx::Public::Full
            )
        },
        syn::Visibility::Restricted(v) => {
            let path = v.path.to_token_stream().to_string();

            match path.as_str() {
                "self" => quote! {
                    ::reflectx::Visibility::Public(
                        ::reflectx::Public::Type
                    )
                },
                "crate" => quote! {
                    ::reflectx::Visiblity::Public(
                        ::reflectx::Public::Crate
                    )
                },
                "super" => quote! {
                    ::reflectx::Visiblity::Public(
                        ::reflectx::Public::Super
                    )
                },
                other => quote! {
                    ::reflectx::Visiblity::Public(
                        ::reflectx::Public::Mod(#other.to_string())
                    )
                },
            }
        }
    };
}
