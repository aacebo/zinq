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
pub trait Parse: Sized + Peek {
    fn parse<P: Parser>(parser: &mut P) -> Result<Self>;
    fn span(&self) -> &Span;
}

///
/// ## Parser
/// a convenient way to conditionally
/// traverse/parse a sequence of data
///
pub trait Parser {
    type Item;

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
    /// ## peek_as
    /// peek to see if a type can be parsed
    ///
    fn peek_as<T: Peek>(&self) -> bool;

    // ///
    // /// ## peek_n
    // /// peek n indices ahead of the current position
    // ///
    // fn peek_n(&self, n: usize) ->

    ///
    /// ## parse
    /// parse a type
    ///
    fn parse<T: Parse>(&mut self) -> Result<T>;

    ///
    /// ## shift_left
    /// shift the current span left by one.
    ///
    fn shift_left(&mut self);

    ///
    /// ## shift_right
    /// shift the current span right by one.
    ///
    fn shift_right(&mut self);

    ///
    /// ## next
    /// advance the end of the span by 1
    ///
    fn next(&mut self);

    ///
    /// ## next_if
    /// advance the end of the span by 1
    /// conditionally
    ///
    fn next_if<P: FnOnce(&Self::Item) -> bool>(&mut self, predicate: P) -> bool;

    ///
    /// ## next_while
    /// advance the end of the span
    /// conditionally until predicate returns
    /// false
    ///
    fn next_while<P: FnOnce(&Self::Item) -> bool>(&mut self, predicate: P) -> usize;
}
