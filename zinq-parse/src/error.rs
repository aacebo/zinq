use zinq_error::Error;

use crate::Span;

#[derive(Debug, Clone)]
pub struct ParseError {
    span: Span,
    inner: Error,
}

impl ParseError {
    #[inline]
    pub fn new(span: Span, inner: Error) -> Self {
        Self { span, inner }
    }

    #[inline]
    pub fn span(&self) -> &Span {
        &self.span
    }

    #[inline]
    pub fn code(&self) -> Option<u32> {
        self.inner.code()
    }

    #[inline]
    pub fn inner(&self) -> &Error {
        &self.inner
    }
}

impl std::fmt::Display for ParseError {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] => {}", self.span(), self.inner())
    }
}

impl std::error::Error for ParseError {
    #[inline]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.inner)
    }
}
