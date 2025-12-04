use macrox::{Element, Error, Ident, StructContext, registry};
use quote::quote;
use syn::spanned::Spanned;

#[derive(Debug, Clone)]
pub struct StructDerive;

impl Element for StructDerive {
    fn render_struct(
        &self,
        context: &mut StructContext,
    ) -> Result<proc_macro2::TokenStream, Error> {
        let target = context.target_mut();
        let ident = &target.ident;
        let mut methods = vec![];

        for field in &target.fields {
            let field_ident = field.ident.clone().unwrap();

            if let None = field.attrs.iter().find(|a| a.meta.path().is_ident("proxy")) {
                continue;
            }

            let fields = match crate::get(&field.ty.ident().to_string()) {
                None => {
                    let message = format!("type '{}' not found", &field.ty.ident());
                    return Err(Error::new(&message).with_span(field.ty.span()));
                }
                Some(entry) => match entry.declare.get() {
                    syn::Item::Struct(s) => s.fields,
                    _ => {
                        return Err(
                            Error::new("proxy type must be struct").with_span(field.ty.span())
                        );
                    }
                },
            };

            for field_ext in &fields {
                if let Some(_) = field_ext
                    .attrs
                    .iter()
                    .find(|a| a.meta.path().is_ident("proxy"))
                {
                    continue;
                }

                let field_ext_ident = field_ext.ident.clone().unwrap();
                let field_ext_ty = &field_ext.ty;

                methods.push(quote! {
                    pub fn #field_ext_ident(&self) -> &#field_ext_ty {
                        &self.#field_ident.#field_ext_ident
                    }
                });
            }
        }

        crate::put(
            target.ident.to_string(),
            registry::TypeEntry::from(syn::Item::Struct(target.clone())),
        );

        if methods.is_empty() {
            return Ok(quote!());
        }

        return Ok(quote! {
            impl #ident {
                #(#methods)*
            }
        });
    }
}
