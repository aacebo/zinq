mod byte_parser;
mod bytes;
mod commit;
mod cursor;
pub mod delta;
mod error;
mod file_meta_data;
mod location;
mod span;
mod tx;

pub use byte_parser::*;
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
    ///
    fn cursor(&self) -> &Cursor;

    ///
    /// ## span
    /// get the current span
    ///
    fn span(&self) -> &Span {
        self.cursor().span()
    }

    ///
    /// ## peek
    /// peek at the next byte
    ///
    fn peek(&self) -> Option<&u8> {
        self.peek_n(1).first()
    }

    ///
    /// ## peek_n
    /// peek n indices ahead of the current position
    ///
    fn peek_n(&self, n: usize) -> &[u8] {
        let (items, _) = self.span().bytes().split_at(n);
        items
    }

    ///
    /// ## peek_at
    /// peek at an item by its index
    ///
    fn peek_at(&self, index: usize) -> Option<&u8> {
        self.span().bytes().get(index)
    }

    ///
    /// ## peek_as
    /// parse a type
    ///
    fn peek_as<T: Peek>(&self) -> bool
    where
        Self: Sized,
    {
        T::peek(self)
    }

    ///
    /// ## error
    /// create an error with a given message
    /// at the current parser location
    ///
    fn error(&self, message: &str) -> Error {
        self.cursor().error(message)
    }

    ///
    /// ## parse
    /// parse an item
    ///
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
    fn parse_as<T: Parse>(&mut self) -> Result<Tx<T>>
    where
        Self: Sized,
    {
        Ok(Tx::from(T::parse(self)?))
    }
}
