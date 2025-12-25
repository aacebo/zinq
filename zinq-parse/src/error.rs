use std::rc::Rc;

use zinq_error::{Error, ErrorCode, ZinqError};

use crate::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    span: Span,
    inner: Error,
}

impl ParseError {
    #[inline]
    pub fn from_error(span: Span, inner: Error) -> Self {
        Self { span, inner }
    }

    #[inline]
    pub fn from_str(span: Span, message: &str) -> Self {
        Self {
            span,
            inner: Error::from_str(message).build(),
        }
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

impl ZinqError for ParseError {
    fn code(&self) -> &ErrorCode {
        self.inner.code()
    }

    fn message(&self) -> Option<&str> {
        self.inner.message()
    }

    fn source(&self) -> Option<&dyn zinq_error::StdError> {
        self.inner.source()
    }

    fn children(&self) -> &[Rc<dyn ZinqError>]
    where
        Self: Sized,
    {
        self.inner.children()
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

impl std::ops::Deref for ParseError {
    type Target = Error;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
