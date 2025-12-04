use proc_macro2::TokenStream;
use syn::spanned::Spanned;

use crate::{Context, EnumContext, StructContext};

///
/// ## Element
/// a component that can parse/render
/// rust syntax
///
pub trait Element {
    ///
    /// ## parse
    /// build the context or return None
    /// to ignore this element
    ///
    fn parse(
        &self,
        input: Option<TokenStream>,
        tokens: TokenStream,
    ) -> Result<Context, crate::Error> {
        let item = syn::parse::<syn::Item>(tokens.into()).unwrap();
        let context = match item {
            syn::Item::Struct(target) => Context::Struct(StructContext { input, target }),
            syn::Item::Enum(target) => Context::Enum(EnumContext { input, target }),
            v => return Err(crate::Error::new("error while parsing element").with_span(v.span())),
        };

        return Ok(context);
    }

    ///
    /// ## parse
    /// build the context or return None
    /// to ignore this element
    ///
    fn parse_derive(&self, tokens: TokenStream) -> Result<Context, crate::Error> {
        let input = syn::parse::<syn::DeriveInput>(tokens.into()).unwrap();
        let context: Context = match input.data {
            syn::Data::Struct(target) => Context::Struct(StructContext {
                input: None,
                target: syn::ItemStruct {
                    attrs: input.attrs.clone(),
                    fields: target.fields.clone(),
                    generics: input.generics.clone(),
                    ident: input.ident.clone(),
                    semi_token: target.semi_token.clone(),
                    struct_token: target.struct_token.clone(),
                    vis: input.vis.clone(),
                },
            }),
            syn::Data::Enum(target) => Context::Enum(EnumContext {
                input: None,
                target: syn::ItemEnum {
                    attrs: input.attrs.clone(),
                    variants: target.variants.clone(),
                    generics: input.generics.clone(),
                    ident: input.ident.clone(),
                    brace_token: target.brace_token.clone(),
                    enum_token: target.enum_token.clone(),
                    vis: input.vis.clone(),
                },
            }),
            _ => {
                return Err(
                    crate::Error::new("error while parsing element").with_span(input.span())
                );
            }
        };

        return Ok(context);
    }

    ///
    /// ## render_enum
    /// render tokens for some enum
    ///
    fn render_enum(&self, _context: &mut EnumContext) -> Result<TokenStream, crate::Error> {
        unimplemented!()
    }

    ///
    /// ## render_struct
    /// render tokens for some struct
    ///
    fn render_struct(&self, _context: &mut StructContext) -> Result<TokenStream, crate::Error> {
        unimplemented!()
    }

    ///
    /// ## render
    /// called with a context and returns a token stream
    ///
    fn render(&self, context: &mut Context) -> Result<TokenStream, crate::Error> {
        return match context {
            Context::Enum(v) => self.render_enum(v),
            Context::Struct(v) => self.render_struct(v),
        };
    }
}
