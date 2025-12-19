mod bytes;
mod cursor;
mod error;
mod file_meta_data;
mod location;
mod span;

pub use bytes::*;
pub use cursor::*;
pub use error::*;
pub use file_meta_data::*;
pub use location::*;
pub use span::*;

use zinq_error::Result;

pub trait Peek {
    fn peek<P: Parser>(parser: &P) -> bool;
}

pub trait Parse: Peek {
    type Output;

    fn parse<P: Parser>(parser: &mut P) -> Result<Self::Output>;
}

pub trait Parser {
    fn cursor(&self) -> &Cursor;
    fn cursor_mut(&mut self) -> &mut Cursor;

    fn span(&self) -> &Span;
    fn peek<T: Peek>(&self) -> bool;
    fn parse<T: Parse>(&mut self) -> Result<T>;
}
