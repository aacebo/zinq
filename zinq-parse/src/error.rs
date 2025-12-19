use crate::Span;

#[derive(Debug, Clone)]
pub struct ParseError<Error: std::error::Error + 'static> {
    span: Span,
    inner: Error,
}

impl<Error: std::error::Error + 'static> ParseError<Error> {
    #[inline]
    pub fn new(span: Span, inner: Error) -> Self {
        Self { span, inner }
    }

    #[inline]
    pub fn span(&self) -> &Span {
        &self.span
    }

    #[inline]
    pub fn inner(&self) -> &Error {
        &self.inner
    }
}

impl<Error: std::error::Error + 'static> std::fmt::Display for ParseError<Error> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner())
    }
}

impl<Error: std::error::Error + 'static> std::error::Error for ParseError<Error> {
    #[inline]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.inner)
    }
}
