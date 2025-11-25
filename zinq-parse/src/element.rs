use crate::Context;

///
/// ## Element
/// a basic modularization of token rendering
/// for proc macros
///
pub trait Element {
    ///
    /// the context type to be passed
    /// into the element for rendering
    ///
    type Context: Context;

    ///
    /// the token output type
    ///
    type Output: quote::ToTokens;

    ///
    /// ## select
    /// called before `render`, returns true if this
    /// element can render the given context
    ///
    fn select(&self, context: &mut Self::Context) -> bool;

    ///
    /// ## render
    /// called with a context and returns a token stream
    ///
    fn render(&self, context: &mut Self::Context) -> Result<Self::Output, crate::Error>;
}
