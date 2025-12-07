use macrox::{Element, Error, StructContext};
use quote::quote;

#[derive(Debug, Clone)]
pub struct StructDerive;

impl Element for StructDerive {
    fn render_struct(
        &self,
        context: &mut StructContext,
    ) -> Result<proc_macro2::TokenStream, Error> {
        let target = context.target();
        let ident = &target.ident;
        let fields_get = target
            .fields
            .iter()
            .map(|field| {
                let field_ident = &field.ident;

                quote! {
                    stringify!(#field_ident) => Some(&self.#field_ident)
                }
            })
            .collect::<Vec<_>>();

        Ok(quote! {
            impl ::configx::Config for #ident {
                fn get(&self, path: &::configx::Path) -> Option<&str> {
                    match path.to_string().as_str() {
                        #(#fields_get, )*
                    }
                }

                fn section(&self, path: &::configx::Path) -> Option<::std::sync::Arc<dyn ::configx::Section>> {
                    todo!()
                }

                fn children(&self) -> Vec<::std::sync::Arc<dyn ::configx::Section>> {
                    vec![]
                }
            }
        })
    }
}
