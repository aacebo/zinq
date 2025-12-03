use proc_macro2::TokenStream;
use syn::spanned::Spanned;

use crate::{Context, EnumContext, StructContext};

///
/// ## Element
/// a basic modularization of token rendering
/// for proc macros
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
    /// ## render
    /// called with a context and returns a token stream
    ///
    fn render(&self, context: &mut Context) -> Result<TokenStream, crate::Error>;
}
