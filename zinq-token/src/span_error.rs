use crate::Span;

#[derive(Debug, Clone)]
pub struct SpanError<Error: std::error::Error + 'static> {
    span: Span,
    inner: Error,
}

impl<Error: std::error::Error + 'static> SpanError<Error> {
    pub fn new(span: Span, inner: Error) -> Self {
        Self { span, inner }
    }

    pub fn span(&self) -> &Span {
        &self.span
    }

    pub fn inner(&self) -> &Error {
        &self.inner
    }
}

impl<Error: std::error::Error + 'static> std::fmt::Display for SpanError<Error> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner())
    }
}

impl<Error: std::error::Error + 'static> std::error::Error for SpanError<Error> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.inner)
    }
}
