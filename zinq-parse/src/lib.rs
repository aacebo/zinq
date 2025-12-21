mod bytes;
mod commit;
mod cursor;
pub mod delta;
mod error;
mod file_meta_data;
mod location;
mod span;
mod tx;

pub use bytes::*;
pub use commit::*;
pub use cursor::*;
pub use error::*;
pub use file_meta_data::*;
pub use location::*;
pub use span::*;
pub use tx::*;

use zinq_error::{Error, Result};

///
/// ## Peek
/// implementers can be peeked by `Parser`
///
pub trait Peek {
    fn peek<P: Parser>(parser: &P) -> bool;
}

///
/// ## Parse
/// implementers can be parsed by `Parser`
///
pub trait Parse: Sized + Peek + std::fmt::Debug + Clone {
    fn parse<P: Parser>(parser: &mut P) -> Result<Self>;
    fn span(&self) -> &Span;
}

///
/// ## Parser
/// a convenient way to conditionally
/// traverse/parse a sequence of data
///
pub trait Parser {
    type Item: Parse + std::fmt::Debug + Clone;

    ///
    /// ## cursor
    /// borrow the parsers `Cursor`
    ///
    fn cursor(&self) -> &Cursor;

    ///
    /// ## cursor_mut
    /// borrow the parsers `Cursor` as mutable,
    /// allowing you to manually manipulate its bounds
    ///
    fn cursor_mut(&mut self) -> &mut Cursor;

    ///
    /// ## span
    /// get the current span
    ///
    #[inline]
    fn span(&self) -> &Span {
        self.cursor().span()
    }

    ///
    /// ## error
    /// create an error with a given message
    /// at the current parser location
    ///
    #[inline]
    fn error(&self, message: &str) -> Error {
        self.cursor().error(message)
    }

    ///
    /// ## peek_as
    /// parse a type
    ///
    #[inline]
    fn peek_as<T: Peek>(&self) -> bool
    where
        Self: Sized,
    {
        T::peek(self)
    }

    ///
    /// ## parse
    /// parse an item
    ///
    #[inline]
    fn parse(&mut self) -> Result<Tx<Self::Item>>
    where
        Self: Sized,
    {
        Ok(Tx::from(Self::Item::parse(self)?))
    }

    ///
    /// ## parse_as
    /// parse a type
    ///
    #[inline]
    fn parse_as<T: Parse>(&mut self) -> Result<Tx<T>>
    where
        Self: Sized,
    {
        Ok(Tx::from(T::parse(self)?))
    }
}
