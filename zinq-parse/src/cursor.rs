use zinq_error::{Error, ErrorBuilder, Result};

use crate::{
    Diagnostic, EOF, ParseError, ParseResult, Span, Tx,
    diagnostic::{self, NOOP},
};

///
/// ## Cursor
/// a mutable `Span` that provides
/// ways to move the `start` and `end`
/// bounds
#[derive(Debug, Default, Clone)]
pub struct Cursor {
    changes: Tx<Span>,
    diagnostics: Vec<Diagnostic>,
}

impl Cursor {
    ///
    /// ## fork
    /// fork the cursor, i.e. clone the cursor
    /// without cloning the existing diagnostics.
    ///
    #[inline]
    pub fn fork(&self) -> Self {
        Self {
            changes: self.changes.clone(),
            diagnostics: vec![],
        }
    }

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
        let end = self.span().end().index();
        &self.span().src()[end + 1..end + n + 1]
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
    /// ## next_n
    /// advance forward by `n` and return the
    /// resulting `Span`
    ///
    #[inline]
    pub fn next_n(&mut self, n: usize) -> Result<()> {
        for _ in 0..n {
            self.next_or_err()?;
        }

        Ok(())
    }

    ///
    /// ## next_or_err
    /// advance or return EOF error
    ///
    #[inline]
    pub fn next_or_err(&mut self) -> Result<&u8> {
        match self.next() {
            None => Err(Error::new().code(EOF).build().into()),
            Some(v) => Ok(v),
        }
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
    pub fn next_while<P: Fn(&u8, &Span) -> bool>(&mut self, predicate: P) -> &Span {
        let mut fork = self.clone();

        while let Some(byte) = fork.peek() {
            if !predicate(byte, fork.span()) {
                break;
            }

            fork.next();
        }

        *self = fork;
        self.span()
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
    /// ## commit
    /// commit the staged changes to the transaction
    ///
    #[inline]
    pub fn commit(&mut self) -> &mut Self {
        let mut span = self.span().clone();
        span.start_mut()
            .seek(self.span().end().index(), self.span().src());
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
        self.diagnostics.push(
            Diagnostic::noop(self.span().clone())
                .children(&other.diagnostics)
                .build(),
        );
        self
    }

    ///
    /// ## error
    /// create an error with a given message
    /// at the current parser location
    ///
    #[inline]
    pub fn error(&mut self, message: &str) -> ErrorBuilder {
        Error::from_error(ParseError::from_str(self.span().clone(), message))
    }

    ///
    /// ## report
    /// emit a diagnostic to be added to the analyzer report
    ///
    #[inline]
    pub fn report(&mut self, code: diagnostic::Code) -> &mut Self {
        self.diagnostics.push(code.at(self.span().clone()).build());
        self
    }

    ///
    /// ## report_as
    /// emit a diagnostic to be added to the analyzer report
    ///
    #[inline]
    pub fn report_as(&mut self, code: diagnostic::Code, message: &str) -> &mut Self {
        self.diagnostics
            .push(code.at(self.span().clone()).message(message).build());
        self
    }

    ///
    /// ## build
    /// build the parse result from the cursor
    ///
    #[inline]
    pub fn build<T>(&self, value: T) -> ParseResult<T> {
        if self.diagnostics.is_empty() {
            return ParseResult::<T>::from_value(value);
        }

        ParseResult {
            value,
            diagnostic: Some(
                NOOP.at(self.span().clone())
                    .children(&self.diagnostics)
                    .build(),
            ),
        }
    }
}

impl From<Span> for Cursor {
    #[inline]
    fn from(span: Span) -> Self {
        Self {
            changes: Tx::<Span>::new(span.clone()),
            diagnostics: vec![],
        }
    }
}

impl std::ops::Deref for Cursor {
    type Target = Span;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.span()
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
        let span = Span::from_bytes(&bytes);
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
        let span = Span::from_bytes(&bytes);
        let mut cursor = span.cursor();

        debug_assert_eq!(cursor.span().bytes(), b"h");
        debug_assert_eq!(cursor.next(), Some(&b'i'));
        debug_assert_eq!(cursor.span().bytes(), b"hi");

        debug_assert_eq!(cursor.shift_next().bytes(), b"i\n");
        debug_assert_eq!(cursor.span().bytes(), b"i\n");

        debug_assert_eq!(cursor.shift_back().bytes(), b"hi");
        debug_assert_eq!(cursor.span().bytes(), b"hi");
    }

    #[test]
    fn should_merge() {
        let bytes = Bytes::from(b"hi\nmy\n\nname\n\n\nis\n\n\n\nbob");
        let span = Span::from_bytes(&bytes);
        let mut cursor = span.cursor();
        let mut fork = cursor.fork();
        let prev = fork.next_while(|b, _| b != &b'n');

        debug_assert_eq!(prev.bytes(), b"hi\nmy\n\n");
        debug_assert_eq!(cursor.span().bytes(), b"h");
        debug_assert_eq!(fork.span().bytes(), b"hi\nmy\n\n");

        cursor.merge(&fork.commit());

        debug_assert_eq!(cursor.span().bytes(), b"\n");
        debug_assert_eq!(fork.span().bytes(), b"\n");
    }

    #[test]
    fn should_advance_by_n() {
        let bytes = Bytes::from(b"b'\n'");
        let span = Span::from_bytes(&bytes);
        let mut cursor = span.cursor();

        debug_assert_eq!(cursor.span().bytes(), b"b");

        cursor.next_n(2).expect("should not error");

        debug_assert_eq!(cursor.span().bytes(), b"b'\n");
    }
}
