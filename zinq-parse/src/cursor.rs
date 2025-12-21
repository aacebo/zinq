use zinq_error::{AnyError, Error, TextError};

use crate::{ParseError, Span};

///
/// ## Cursor
/// a mutable `Span` that provides
/// ways to move the `start` and `end`
/// bounds
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Cursor {
    span: Span,
}

impl Cursor {
    ///
    /// ## span
    /// get the current span
    ///
    pub fn span(&self) -> &Span {
        &self.span
    }

    ///
    /// ## backward
    /// move both the `start` and `end` bounds
    /// backwards 1
    ///
    pub fn backward(&mut self) -> &Span {
        todo!()
    }

    ///
    /// ## forward
    /// move both the `start` and `end` bounds
    /// forward 1
    ///
    pub fn forward(&mut self) -> &Span {
        &self.span
    }

    ///
    /// ## back
    /// decrement the `start` of the span by 1
    ///
    pub fn back(&mut self) -> Option<&u8> {
        if self.span.sof() {
            return None;
        }

        self.span.start.back(&self.span.bytes);
        Some(self.span.first())
    }

    ///
    /// ## next
    /// advance the `end` of the span by 1
    ///
    pub fn next(&mut self) -> Option<&u8> {
        if self.span.eof() {
            return None;
        }

        self.span.end.next(&self.span.bytes);
        Some(self.span.last())
    }

    ///
    /// ## next_if
    /// advance the end of the span by 1
    /// conditionally
    ///
    pub fn next_if<P: FnOnce(&u8, &Span) -> bool>(&mut self, predicate: P) -> Option<&u8> {
        if predicate(self.span().last(), self.span()) {
            return self.next();
        }

        None
    }

    ///
    /// ## next_while
    /// advance the end of the span
    /// conditionally until predicate returns
    /// false
    ///
    pub fn next_while<P: Fn(&&u8, &&Span) -> bool>(&mut self, predicate: P) -> Option<&Span> {
        let mut fork = self.clone();

        for byte in self.span().bytes().iter() {
            if !predicate(&byte, &fork.span()) {
                break;
            }

            fork.next();
        }

        *self = fork;
        Some(self.span())
    }

    ///
    /// ## error
    /// create an error with a given message
    /// at the current parser location
    ///
    pub fn error(&self, message: &str) -> Error {
        AnyError::new(ParseError::new(
            self.span().clone(),
            TextError::from(message),
        ))
        .into()
    }
}

impl From<Span> for Cursor {
    fn from(span: Span) -> Self {
        Self { span }
    }
}
