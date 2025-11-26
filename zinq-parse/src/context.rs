///
/// ## Context
/// the context that gets parsed and passed into
/// `Element::render()`
///
pub trait Context {
    ///
    /// the attribute macro input to be parsed
    ///
    type Input: syn::parse::Parse;

    ///
    /// the rust type to be parsed
    ///
    type Target: syn::parse::Parse + quote::ToTokens;

    ///
    /// ## args
    /// parse and return the context args
    ///
    fn input(&self) -> Option<&Self::Input>;

    ///
    /// ## target
    /// borrow a reference of the parsed context target
    ///
    fn target(&self) -> &Self::Target;

    ///
    /// ## target_mut
    /// borrow a mutable reference of the parsed context target
    ///
    fn target_mut(&mut self) -> &mut Self::Target;
}
