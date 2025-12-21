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
    #[inline]
    pub fn span(&self) -> &Span {
        &self.span
    }

    ///
    /// ## peek
    /// peek at the next byte
    ///
    #[inline]
    pub fn peek(&self) -> Option<&u8> {
        self.peek_n(1).first()
    }

    ///
    /// ## peek_n
    /// peek n indices ahead of the current position
    ///
    #[inline]
    pub fn peek_n(&self, n: usize) -> &[u8] {
        let (items, _) = self.span().bytes().split_at(n);
        items
    }

    ///
    /// ## peek_at
    /// peek at an item by its index
    ///
    #[inline]
    pub fn peek_at(&self, index: usize) -> Option<&u8> {
        self.span().bytes().get(index)
    }

    ///
    /// ## backward
    /// move both the `start` and `end` bounds
    /// backwards 1
    ///
    #[inline]
    pub fn backward(&mut self) -> &Span {
        let mut span = self.span.clone();
        span.start_mut().back(self.span.src());
        span.end_mut().back(self.span.src());
        self.span = span;
        &self.span
    }

    ///
    /// ## forward
    /// move both the `start` and `end` bounds
    /// forward 1
    ///
    #[inline]
    pub fn forward(&mut self) -> &Span {
        let mut span = self.span.clone();
        span.start_mut().next(self.span.src());
        span.end_mut().next(self.span.src());
        self.span = span;
        &self.span
    }

    ///
    /// ## back
    /// decrement the `start` of the span by 1
    ///
    #[inline]
    pub fn back(&mut self) -> Option<&u8> {
        if self.span.sof() {
            return None;
        }

        let mut span = self.span.clone();
        span.start_mut().back(self.span.src());
        self.span = span;
        Some(self.span.first())
    }

    ///
    /// ## next
    /// advance the `end` of the span by 1
    ///
    #[inline]
    pub fn next(&mut self) -> Option<&u8> {
        if self.span.eof() {
            return None;
        }

        let mut span = self.span.clone();
        span.end_mut().next(self.span.src());
        self.span = span;
        Some(self.span.last())
    }

    ///
    /// ## next_if
    /// advance the end of the span by 1
    /// conditionally
    ///
    #[inline]
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
    #[inline]
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
    /// ## skip
    /// move the end of the span forward by 1
    /// without returning the value
    ///
    #[inline]
    pub fn skip(&mut self) -> &mut Self {
        self.next();
        self
    }

    ///
    /// ## skip_n
    /// skip `n` bytes by moving the `end` of the span
    /// forward by `n`
    ///
    #[inline]
    pub fn skip_n(&mut self, n: usize) -> &mut Self {
        for _ in 0..n {
            self.skip();
        }

        self
    }

    ///
    /// ## error
    /// create an error with a given message
    /// at the current parser location
    ///
    #[inline]
    pub fn error(&self, message: &str) -> Error {
        AnyError::new(ParseError::new(
            self.span().clone(),
            TextError::from(message),
        ))
        .into()
    }
}

impl From<Span> for Cursor {
    #[inline]
    fn from(span: Span) -> Self {
        Self { span }
    }
}

// #[cfg(test)]
// mod test {
//     #[test]
//     fn should_create_cursor() {
//         let bytes = Bytes::from(b"hi\nmy\n\nname\n\n\nis\n\n\n\nbob");
//     }
// }
