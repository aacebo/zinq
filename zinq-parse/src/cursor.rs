use zinq_error::Error;

use crate::{ParseError, Span, Tx};

///
/// ## Cursor
/// a mutable `Span` that provides
/// ways to move the `start` and `end`
/// bounds
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Cursor {
    changes: Tx<Span>,
}

impl Cursor {
    ///
    /// ## span
    /// get the current span
    ///
    #[inline]
    pub fn span(&self) -> &Span {
        self.changes.last()
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
        let (_, items) = self.span().src().split_at(self.span().end().index() + n);
        items
    }

    ///
    /// ## peek_at
    /// peek at an item by its index
    ///
    #[inline]
    pub fn peek_at(&self, index: usize) -> Option<&u8> {
        self.span().src().get(index)
    }

    ///
    /// ## shift_back
    /// move both the `start` and `end` bounds
    /// backwards 1
    ///
    #[inline]
    pub fn shift_back(&mut self) -> &Span {
        let mut span = self.span().clone();
        span.start_mut().back(self.span().src());
        span.end_mut().back(self.span().src());
        self.changes.next(span);
        self.span()
    }

    ///
    /// ## shift_next
    /// move both the `start` and `end` bounds
    /// forward 1
    ///
    #[inline]
    pub fn shift_next(&mut self) -> &Span {
        let mut span = self.span().clone();
        span.start_mut().next(self.span().src());
        span.end_mut().next(self.span().src());
        self.changes.next(span);
        self.span()
    }

    ///
    /// ## back
    /// decrement the `start` of the span by 1
    ///
    #[inline]
    pub fn back(&mut self) -> Option<&u8> {
        if self.span().sof() {
            return None;
        }

        let mut span = self.span().clone();
        span.start_mut().back(self.span().src());
        self.changes.next(span);
        Some(self.span().first())
    }

    ///
    /// ## next
    /// advance the `end` of the span by 1
    ///
    #[inline]
    pub fn next(&mut self) -> Option<&u8> {
        if self.span().eof() {
            return None;
        }

        let mut span = self.span().clone();
        span.end_mut().next(self.span().src());
        self.changes.next(span);
        Some(self.span().last())
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
        Error::from(ParseError::from_str(self.span().clone(), message)).build()
    }

    ///
    /// ## commit
    /// commit the staged changes to the transaction
    ///
    #[inline]
    pub fn commit(&mut self) -> &mut Self {
        let mut span = self.span().clone();
        span.start_mut()
            .seek(self.span().end().index(), self.span().bytes());
        self.changes.next(span);
        self
    }

    ///
    /// ## merge
    /// merge a forked `Cursor`
    ///
    #[inline]
    pub fn merge(&mut self, other: &Self) -> &mut Self {
        self.changes.next(other.span().clone());
        self
    }
}

impl From<Span> for Cursor {
    #[inline]
    fn from(span: Span) -> Self {
        Self {
            changes: Tx::<Span>::new(span.clone()),
        }
    }
}

impl std::fmt::Display for Cursor {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

#[cfg(test)]
mod test {
    use crate::{Bytes, Span};

    #[test]
    fn should_peek() {
        let bytes = Bytes::from(b"hi\nmy\n\nname\n\n\nis\n\n\n\nbob");
        let span = Span::from_bytes(&bytes).expect("expected span");
        let mut cursor = span.cursor();

        debug_assert_eq!(cursor.span().bytes(), b"h");
        debug_assert_eq!(cursor.next(), Some(&b'i'));
        debug_assert_eq!(cursor.span().bytes(), b"hi");

        debug_assert_eq!(cursor.peek(), Some(&b'\n'));
        debug_assert_eq!(cursor.span().bytes(), b"hi");
        debug_assert_eq!(cursor.peek_at(8), Some(&b'a'));
        debug_assert_eq!(cursor.span().bytes(), b"hi");
    }

    #[test]
    fn should_shift() {
        let bytes = Bytes::from(b"hi\nmy\n\nname\n\n\nis\n\n\n\nbob");
        let span = Span::from_bytes(&bytes).expect("expected span");
        let mut cursor = span.cursor();

        debug_assert_eq!(cursor.span().bytes(), b"h");
        debug_assert_eq!(cursor.next(), Some(&b'i'));
        debug_assert_eq!(cursor.span().bytes(), b"hi");

        debug_assert_eq!(cursor.shift_next().bytes(), b"i\n");
        debug_assert_eq!(cursor.span().bytes(), b"i\n");

        debug_assert_eq!(cursor.shift_back().bytes(), b"hi");
        debug_assert_eq!(cursor.span().bytes(), b"hi");
    }
}
