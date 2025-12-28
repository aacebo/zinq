use std::rc::Rc;

use zinq_error::{BAD_ARGUMENTS, Error, ZinqError};
use zinq_parse::{ParseError, Span};

#[derive(Debug, Clone)]
pub struct TokenExpectedError {
    inner: ParseError,
}

impl TokenExpectedError {
    #[inline]
    pub fn from(expected: &[u8], received: &[u8], span: Span) -> Self {
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

impl std::fmt::Display for TokenExpectedError {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner())
    }
}

impl std::error::Error for TokenExpectedError {
    #[inline]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.inner)
    }
}

impl std::ops::Deref for TokenExpectedError {
    type Target = ZinqError;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Into<ZinqError> for TokenExpectedError {
    fn into(self) -> ZinqError {
        ZinqError::Std(Rc::new(self))
    }
}
