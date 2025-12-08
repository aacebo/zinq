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
                    stringify!(#field_ident) => Some(format!("{:#?}", self.#field_ident))
                }
            })
            .collect::<Vec<_>>();

        Ok(quote! {
            impl ::configx::Config for #ident {
                fn value(&self) -> String {
                    unimplemented!()
                }

                fn get(&self, path: &::configx::Path) -> Option<String> {
                    match path.to_string().as_str() {
                        #(#fields_get, )*
                        _ => None,
                    }
                }

                fn section<'a>(&self, path: &::configx::Path) -> Option<&'a dyn ::configx::Section> {
                    unimplemented!()
                }

                fn children(&self) -> Vec<::std::sync::Arc<dyn ::configx::Section>> {
                    vec![]
                }
            }
        })
    }
}
