mod bytes;
mod commit;
mod cursor;
pub mod delta;
pub mod diagnostic;
mod error;
mod file_meta_data;
mod location;
mod result;
mod span;
mod tx;

pub use bytes::*;
pub use commit::*;
pub use cursor::*;
pub use diagnostic::Diagnostic;
pub use error::*;
pub use file_meta_data::*;
pub use location::*;
pub use result::*;
pub use span::*;
pub use tx::*;

use zinq_error::Result;

///
/// ## Peek
/// implementers can be peeked by `Parser`
///
pub trait Peek<P: Parser> {
    fn peek(cursor: &Cursor, parser: &P) -> Result<bool>;
}

///
/// ## Parse
/// implementers can be parsed by `Parser`
///
pub trait Parse<P: Parser>: Peek<P> + std::fmt::Debug + Clone {
    fn parse(cursor: &mut Cursor, parser: &mut P) -> Result<Self>;
    fn span(&self) -> &Span;
}

///
/// ## Parser
/// a convenient way to conditionally
/// traverse/parse a sequence of data
///
pub trait Parser: Sized {
    type Item: Parse<Self> + std::fmt::Debug + Clone;

    ///
    /// ## peek_as
    /// parse a type
    ///
    #[inline]
    fn peek_as<T: Peek<Self>>(&self, cursor: &Cursor) -> Result<bool>
    where
        Self: Sized,
    {
        if cursor.peek()?.is_ascii_whitespace() {
            return self.peek_as::<T>(cursor.fork().shift_next()?);
        }

        T::peek(cursor, self)
    }

    ///
    /// ## parse
    /// parse an item
    ///
    #[inline]
    fn parse(&mut self, cursor: &mut Cursor) -> Result<Self::Item>
    where
        Self: Sized,
    {
        if cursor.peek()?.is_ascii_whitespace() {
            return self.parse(cursor.shift_next()?);
        }

        let mut fork = cursor.fork();
        let value = Self::Item::parse(&mut fork, self)?;
        cursor.merge(fork.commit());
        Ok(value)
    }

    ///
    /// ## parse_as
    /// parse a type
    ///
    #[inline]
    fn parse_as<T: Parse<Self>>(&mut self, cursor: &mut Cursor) -> Result<T>
    where
        Self: Sized,
    {
        if cursor.peek()?.is_ascii_whitespace() {
            return self.parse_as::<T>(cursor.shift_next()?);
        }

        let mut fork = cursor.fork();
        let value = T::parse(&mut fork, self)?;
        cursor.merge(fork.commit());
        Ok(value)
    }
}

impl<T: Peek<P>, P: Parser> Peek<P> for Option<T> {
    fn peek(_: &Cursor, _: &P) -> Result<bool> {
        Ok(true)
    }
}

impl<T: Parse<P>, P: Parser> Parse<P> for Option<T> {
    fn parse(cursor: &mut Cursor, parser: &mut P) -> Result<Self> {
        if !parser.peek_as::<T>(cursor).unwrap_or(false) {
            return Ok(Self::None);
        }

        Ok(Some(parser.parse_as::<T>(cursor)?))
    }

    fn span(&self) -> &Span {
        match self {
            None => panic!("attempted to call Parse::span on None"),
            Some(v) => v.span(),
        }
    }
}

impl<T: Peek<P>, P: Parser> Peek<P> for Box<T> {
    fn peek(cursor: &Cursor, parser: &P) -> Result<bool> {
        parser.peek_as::<T>(cursor)
    }
}

impl<T: Parse<P>, P: Parser> Parse<P> for Box<T> {
    fn parse(cursor: &mut Cursor, parser: &mut P) -> Result<Self> {
        Ok(Box::new(parser.parse_as::<T>(cursor)?))
    }

    fn span(&self) -> &Span {
        self.as_ref().span()
    }
}
