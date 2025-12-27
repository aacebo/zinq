use std::ops::Index;

use zinq_error::{Error, Result, ZinqError, ZinqErrorCode};

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
    /// ## tx
    /// the transaction state
    ///
    #[inline]
    pub fn tx(&self) -> &Tx<Span> {
        &self.changes
    }

    ///
    /// ## fork
    /// fork the cursor, i.e. clone the cursor
    /// without cloning the existing diagnostics.
    ///
    #[inline]
    pub fn fork(&self) -> Self {
        Self {
            changes: Tx::new(self.span().clone()),
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
    pub fn peek(&self) -> Result<&u8> {
        Ok(self.peek_n(1)?.first().expect("expected at least one byte"))
    }

    ///
    /// ## peek_n
    /// peek n indices ahead of the current position
    ///
    #[inline]
    pub fn peek_n(&self, n: usize) -> Result<&[u8]> {
        let end = self.span().end().index();

        if end + n > self.src().len() {
            return Err(self.error(EOF, "end of input stream"));
        }

        Ok(&self.span().src()[end..end + n])
    }

    ///
    /// ## peek_at
    /// peek at an item by its index
    ///
    #[inline]
    pub fn peek_at(&self, index: usize) -> Result<&u8> {
        if index > self.span().src().len() - 1 {
            return Err(self.error(EOF, "end of input stream"));
        }

        Ok(self.span().src().index(index))
    }

    ///
    /// ## shift_back
    /// move both the `start` and `end` bounds
    /// backwards 1
    ///
    #[inline]
    pub fn shift_back(&mut self) -> Result<&mut Self> {
        let mut span = self.span().clone();

        if !span.start_mut().back(self.span().src()) {
            return Err(self.error(EOF, "end of input stream"));
        }

        if !span.end_mut().back(self.span().src()) {
            return Err(self.error(EOF, "end of input stream"));
        }

        self.changes.next(span);
        Ok(self)
    }

    ///
    /// ## shift_next
    /// move both the `start` and `end` bounds
    /// forward 1
    ///
    #[inline]
    pub fn shift_next(&mut self) -> Result<&mut Self> {
        let mut span = self.span().clone();

        if !span.start_mut().next(self.span().src()) {
            return Err(self.error(EOF, "end of input stream"));
        }

        if !span.end_mut().next(self.span().src()) {
            return Err(self.error(EOF, "end of input stream"));
        }

        self.changes.next(span);
        Ok(self)
    }

    ///
    /// ## back
    /// decrement the `start` of the span by 1
    ///
    #[inline]
    pub fn back(&mut self) -> Result<&mut Self> {
        if self.span().sof() {
            return Err(self.error(EOF, "end of input stream"));
        }

        let mut span = self.span().clone();
        span.start_mut().back(self.span().src());
        self.changes.next(span);
        Ok(self)
    }

    ///
    /// ## next
    /// advance the `end` of the span by 1
    ///
    #[inline]
    pub fn next(&mut self) -> Result<&mut Self> {
        if self.span().eof() {
            return Err(self.error(EOF, "end of input stream"));
        }

        let mut span = self.span().clone();
        span.end_mut().next(self.span().src());
        self.changes.next(span);
        Ok(self)
    }

    ///
    /// ## next_n
    /// advance forward by `n` and return the
    /// resulting `Span`
    ///
    #[inline]
    pub fn next_n(&mut self, n: usize) -> Result<&mut Self> {
        for _ in 0..n {
            self.next()?;
        }

        Ok(self)
    }

    ///
    /// ## next_if
    /// advance the end of the span by 1
    /// conditionally
    ///
    #[inline]
    pub fn next_if<P: FnOnce(&u8, &Span) -> bool>(&mut self, predicate: P) -> Result<&mut Self> {
        if predicate(self.peek()?, self.span()) {
            return self.next();
        }

        Ok(self)
    }

    ///
    /// ## next_while
    /// advance the end of the span
    /// conditionally until predicate returns
    /// false
    ///
    #[inline]
    pub fn next_while<P: Fn(&u8, &Span) -> bool>(&mut self, predicate: P) -> Result<&mut Self> {
        let mut fork = self.clone();

        while let Ok(byte) = fork.peek() {
            if !predicate(byte, fork.span()) {
                break;
            }

            fork.next()?;
        }

        *self = fork;
        Ok(self)
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
        self.changes.append(other.tx().clone());

        if !other.diagnostics.is_empty() {
            self.diagnostics.push(
                Diagnostic::noop(self.span().clone())
                    .children(&other.diagnostics)
                    .build(),
            );
        }

        self
    }

    ///
    /// ## error
    /// create an error with a given message
    /// at the current parser location
    ///
    #[inline]
    pub fn error(&self, code: ZinqErrorCode, message: &str) -> ZinqError {
        ParseError::from_error(
            self.span().clone(),
            Error::new().code(code).message(message).build().into(),
        )
        .into()
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
    use zinq_error::Result;

    use crate::{Bytes, Span};

    #[test]
    fn should_peek() -> Result<()> {
        let bytes = Bytes::from(b"hi\nmy\n\nname\n\n\nis\n\n\n\nbob");
        let span = Span::from_bytes(&bytes);
        let mut cursor = span.cursor();

        debug_assert_eq!(cursor.next()?.bytes(), b"h");
        debug_assert_eq!(cursor.next()?.last(), &b'i');
        debug_assert_eq!(cursor.bytes(), b"hi");

        debug_assert_eq!(cursor.peek()?, &b'\n');
        debug_assert_eq!(cursor.bytes(), b"hi");
        debug_assert_eq!(cursor.peek_at(8)?, &b'a');
        debug_assert_eq!(cursor.bytes(), b"hi");

        Ok(())
    }

    #[test]
    fn should_shift() -> Result<()> {
        let bytes = Bytes::from(b"hi\nmy\n\nname\n\n\nis\n\n\n\nbob");
        let span = Span::from_bytes(&bytes);
        let mut cursor = span.cursor();

        debug_assert_eq!(cursor.next()?.bytes(), b"h");
        debug_assert_eq!(cursor.next()?.last(), &b'i');
        debug_assert_eq!(cursor.bytes(), b"hi");

        debug_assert_eq!(cursor.shift_next()?.bytes(), b"i\n");
        debug_assert_eq!(cursor.bytes(), b"i\n");

        debug_assert_eq!(cursor.shift_back()?.bytes(), b"hi");
        debug_assert_eq!(cursor.bytes(), b"hi");

        Ok(())
    }

    #[test]
    fn should_merge() -> Result<()> {
        let bytes = Bytes::from(b"hi\nmy\n\nname\n\n\nis\n\n\n\nbob");
        let span = Span::from_bytes(&bytes);
        let mut cursor = span.cursor();
        let mut fork = cursor.next()?.fork();

        fork.next_while(|b, _| b != &b'n')?;

        debug_assert_eq!(fork.bytes(), b"hi\nmy\n\n");
        debug_assert_eq!(cursor.bytes(), b"h");

        cursor.merge(fork.commit());

        debug_assert_eq!(cursor.next()?.bytes(), b"n");
        debug_assert_eq!(fork.span().bytes(), b"");

        Ok(())
    }

    #[test]
    fn should_advance_by_n() -> Result<()> {
        let bytes = Bytes::from(b"b'\n'");
        let span = Span::from_bytes(&bytes);
        let mut cursor = span.cursor();

        debug_assert_eq!(cursor.next()?.bytes(), b"b");

        cursor.next_n(2).expect("should not error");

        debug_assert_eq!(cursor.bytes(), b"b'\n");

        Ok(())
    }
}
