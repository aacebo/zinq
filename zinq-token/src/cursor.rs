use zinq_error::{AnyError, Error, Result, TextError};

use crate::{Location, Span, SpanError, Token};

#[derive(Debug, Clone)]
pub struct Cursor {
    span: Span,
}

impl Cursor {
    pub fn sof(&self) -> bool {
        self.span.end == self.span.start
    }

    pub fn eof(&self) -> bool {
        self.span.end.index() == self.span.file.bytes().len()
    }

    pub fn span(&self) -> &Span {
        &self.span
    }

    pub fn error(&self, message: &str) -> Error {
        AnyError::new(SpanError::new(self.span.clone(), TextError::from(message))).into()
    }

    pub fn matches(&self, bytes: &[u8]) -> bool {
        self.span.bytes() == bytes
    }

    pub fn next(self) -> Self {
        let index = self.span.end.index() + 1;
        let mut line = self.span.end.line();
        let mut column = self.span.end.column();

        if self.span.file.bytes()[index] == b'\n' {
            line += 1;
            column = 0;
        }

        Self {
            span: Span {
                start: self.span.start,
                end: Location {
                    index,
                    line,
                    column,
                },
                file: self.span.file,
            },
        }
    }

    pub fn parse<T: Token>(self) -> Result<(T, Self)> {
        let next = self.clone().next();
        let token = T::parse(self)?;

        Ok((token, next))
    }
}

impl From<Span> for Cursor {
    fn from(span: Span) -> Self {
        Self { span }
    }
}

impl std::fmt::Display for Cursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}
