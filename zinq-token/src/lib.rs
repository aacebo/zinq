mod cursor;
mod location;
pub mod source;
mod span;
mod span_error;
mod stream;
pub mod tokens;

pub use cursor::*;
pub use location::*;
pub use span::*;
pub use span_error::*;
pub use stream::*;
use zinq_error::Result;

pub trait Token: Sized {
    ///
    /// ## peek
    /// check if the next token is this token type
    ///
    fn peek(cursor: Cursor) -> bool;

    ///
    /// ## parse
    /// parse this token type using a provided stream of tokens
    ///
    fn parse(cursor: Cursor) -> Result<Self>;

    ///
    /// ## span
    /// get the span of the token
    ///
    fn span(&self) -> &Span;
}
