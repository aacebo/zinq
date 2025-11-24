mod struct_element;
mod trait_element;

use proc_macro2::TokenStream;
pub use struct_element::*;
pub use trait_element::*;

pub trait Element {
    type Context: Context;

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
    fn render(&self, context: &mut Self::Context) -> Result<TokenStream, crate::Error>;
}

pub trait Context {
    type Args: Clone + syn::parse::Parse + quote::ToTokens;
    type Value: quote::ToTokens;

    ///
    /// ## args
    /// parse and return the context args
    ///
    fn args(&self) -> Result<Self::Args, crate::Error>;

    ///
    /// ## value
    /// borrow a reference of the parsed context value
    ///
    fn value(&self) -> &Self::Value;

    ///
    /// ## value_mut
    /// borrow a mutable reference of the parsed context value
    ///
    fn value_mut(&mut self) -> &mut Self::Value;
}
