use std::rc::Rc;

use zinq_error::{BAD_ARGUMENTS, Error, ZinqError};
use zinq_parse::{ParseError, Span};

#[derive(Debug, Clone)]
pub struct TokenMismatchError {
    inner: ParseError,
}

impl TokenMismatchError {
    #[inline]
    pub fn from_types(expected: &str, received: &str, span: Span) -> Self {
        Self {
            inner: ParseError::from_error(
                span,
                Error::new()
                    .code(BAD_ARGUMENTS)
                    .message(&format!("expected '{}', received '{}'", expected, received,))
                    .build()
                    .into(),
            ),
        }
    }

    #[inline]
    pub fn from_bytes(expected: &[u8], received: &[u8], span: Span) -> Self {
        Self {
            inner: ParseError::from_error(
                span,
                Error::new()
                    .code(BAD_ARGUMENTS)
                    .message(&format!(
                        "expected '{}', received '{}'",
                        Span::from_bytes(expected),
                        Span::from_bytes(received)
                    ))
                    .build()
                    .into(),
            ),
        }
    }

    #[inline]
    pub fn span(&self) -> &Span {
        &self.inner.span()
    }

    #[inline]
    pub fn inner(&self) -> &ParseError {
        &self.inner
    }
}

impl std::fmt::Display for TokenMismatchError {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner())
    }
}

impl std::error::Error for TokenMismatchError {
    #[inline]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.inner)
    }
}

impl std::ops::Deref for TokenMismatchError {
    type Target = ZinqError;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Into<ZinqError> for TokenMismatchError {
    fn into(self) -> ZinqError {
        ZinqError::Std(Rc::new(self))
    }
}
