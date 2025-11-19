use quote::quote;

pub fn render(input: &syn::DeriveInput, _: &syn::DataEnum) -> proc_macro2::TokenStream {
    let name = &input.ident;

    return quote! {
        impl std::error::Error for #name {
            fn source(&self) -> Option<&(dyn ::std::error::Error + 'static)> {
                return None;
            }
        }
    };
}
