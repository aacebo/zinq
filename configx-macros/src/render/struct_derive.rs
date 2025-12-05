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
        let fields = target
            .fields
            .iter()
            .map(|field| {
                let field_ident = &field.ident;

                quote! {
                    stringify!(#field_ident) => Some(Box::new(self.#field_ident))
                }
            })
            .collect::<Vec<_>>();

        Ok(quote! {
            impl ::configx::Config for #ident {
                fn name(&self) -> &str {
                    return stringify!(#ident);
                }

                fn iter(&self) -> std::vec::IntoIter<(::configx::Key, Box<dyn ::configx::Config>)> {
                    return vec![].into_iter();
                }

                fn get(&self, key: &::configx::Key) -> Option<Box<dyn ::configx::Config>> {
                    return match key {
                        ::configx::Key::String(v) => match v.as_str() {
                            #(#fields, )*
                        },
                        _ => None,
                    };
                }
            }
        })
    }
}
