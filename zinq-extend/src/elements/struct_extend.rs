use quote::quote;
use zinq_parse::{Element, Error, StructContext, registry};

#[derive(Debug, Clone)]
pub struct StructExtend;

impl Element for StructExtend {
    fn render_struct(
        &self,
        context: &mut StructContext,
    ) -> Result<proc_macro2::TokenStream, Error> {
        let input = context.input_required::<crate::Input>();
        let target = context.target_mut();
        let ident = &target.ident;
        let mut methods = proc_macro2::TokenStream::new();

        for name in input.0.iter() {
            let fields_to_add = match registry::get(&name.to_string()) {
                None => {
                    let message = format!("type '{}' not found", &name);
                    return Err(Error::new(&message).with_span(name.span()));
                }
                Some(entry) => match entry.declare.get() {
                    syn::Item::Struct(s) => s.fields.clone(),
                    _ => {
                        return Err(Error::new("extend type must be struct").with_span(name.span()));
                    }
                },
            };

            match fields_to_add {
                syn::Fields::Named(named_fields) => match &mut target.fields {
                    syn::Fields::Named(fields) => {
                        fields.named.extend(named_fields.named.clone());
                        methods.extend(named_fields.named.iter().map(|field| {
                            let field_ident = field.ident.clone().unwrap();
                            let field_ty = &field.ty;

                            quote! {
                                pub fn #field_ident(&self) -> &#field_ty {
                                    return &self.#field_ident;
                                }
                            }
                        }));
                    }
                    _ => {
                        return Err(Error::new(
                            "structs can only extend structs with the same field layout",
                        )
                        .with_span(name.span()));
                    }
                },
                syn::Fields::Unnamed(unnamed_fields) => match &mut target.fields {
                    syn::Fields::Unnamed(fields) => {
                        fields.unnamed.extend(unnamed_fields.unnamed.clone());
                    }
                    _ => {
                        return Err(Error::new(
                            "structs can only extend structs with the same field layout",
                        )
                        .with_span(name.span()));
                    }
                },
                _ => {}
            };
        }

        registry::put(
            target.ident.to_string(),
            registry::TypeEntry::from(syn::Item::Struct(target.clone())),
        );

        return Ok(quote! {
            #target

            impl #ident {
                #methods
            }
        });
    }
}
