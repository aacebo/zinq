use std::rc::Rc;

use zinq_error::{Error, ZinqError, ZinqErrorCode};

use crate::Span;

#[derive(Debug, Clone)]
pub struct ParseError {
    span: Span,
    inner: ZinqError,
}

impl ParseError {
    #[inline]
    pub fn from_error(span: Span, inner: ZinqError) -> Self {
        Self { span, inner }
    }

    #[inline]
    pub fn from_str(span: Span, message: &str) -> Self {
        Self {
            span,
            inner: Error::from_str(message).build().into(),
        }
    }

    #[inline]
    pub fn span(&self) -> &Span {
        &self.span
    }

    #[inline]
    pub fn inner(&self) -> &ZinqError {
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

impl std::ops::Deref for ParseError {
    type Target = ZinqError;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Into<ZinqError> for ParseError {
    fn into(self) -> ZinqError {
        ZinqError::Std(Rc::new(self))
    }
}

pub const EOF: ZinqErrorCode = ZinqErrorCode {
    id: 10,
    name: "EndOfFile",
    description: "end of input stream.",
};
