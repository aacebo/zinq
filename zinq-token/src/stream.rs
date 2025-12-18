use crate::tokens;

///
/// ## TokenStream
/// a stream used to parse tokens
///
#[derive(Debug, Clone)]
pub struct TokenStream(pub(crate) Vec<tokens::Any>);

impl TokenStream {
    #[inline]
    pub fn new() -> Self {
        Self(vec![])
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}
