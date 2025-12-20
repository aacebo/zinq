mod byte_parser;
mod bytes;
mod commit;
pub mod delta;
mod error;
mod file_meta_data;
mod location;
mod span;
mod tx;

pub use byte_parser::*;
pub use bytes::*;
pub use commit::*;
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
    /// ## error
    /// create an error with a given message
    /// at the current parser location
    ///
    fn error(&self, message: &str) -> Error;

    ///
    /// ## span
    /// get the current span
    ///
    fn span(&self) -> &Span;

    ///
    /// ## fork
    /// fork the parser
    ///
    fn fork(&self) -> Self;

    ///
    /// ## peek
    /// peek at the next byte
    ///
    fn peek(&self) -> &Self::Item;

    ///
    /// ## peek_as
    /// peek to see if a type can be parsed
    ///
    fn peek_as<T: Peek>(&self) -> bool;

    ///
    /// ## peek_n
    /// peek n indices ahead of the current position
    ///
    fn peek_n(&self, n: usize) -> &[Self::Item];

    ///
    /// ## peek_n
    /// peek n indices ahead of the current position
    ///
    fn peek_at(&self, index: usize) -> &Self::Item;

    ///
    /// ## parse
    /// parse an item
    ///
    fn parse(&mut self) -> Result<Tx<Self::Item>>;

    ///
    /// ## parse_as
    /// parse a type
    ///
    fn parse_as<T: Parse>(&mut self) -> Result<Tx<T>>;

    ///
    /// ## shift_left
    /// shift the current span left by one.
    ///
    fn shift_left(&mut self) -> Tx<Self::Item>;

    ///
    /// ## shift_right
    /// shift the current span right by one.
    ///
    fn shift_right(&mut self) -> Tx<Self::Item>;

    ///
    /// ## next
    /// advance the end of the span by 1
    ///
    fn next(&mut self) -> Tx<Self::Item>;

    ///
    /// ## next_if
    /// advance the end of the span by 1
    /// conditionally
    ///
    fn next_if<P: FnOnce(&Self::Item) -> bool>(&mut self, predicate: P) -> Tx<Self::Item>;

    ///
    /// ## next_while
    /// advance the end of the span
    /// conditionally until predicate returns
    /// false
    ///
    fn next_while<P: FnOnce(&Self::Item) -> bool>(&mut self, predicate: P) -> Tx<Self::Item>;
}
