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
/// implementers can be peeked by a `ZinqParser`
///
pub trait Peek {
    fn peek(cursor: &Cursor, parser: &ZinqParser) -> Result<bool>;
}

///
/// ## Parse
/// implementers can be parsed by a `ZinqParser`
///
pub trait Parse: Spanned + Peek + std::fmt::Debug + Clone {
    fn parse(cursor: &mut Cursor, parser: &mut ZinqParser) -> Result<Self>;
}

///
/// ## Parser
/// a convenient way to conditionally
/// traverse/parse a sequence of data
///
#[derive(Debug, Clone)]
pub struct ZinqParser;

impl ZinqParser {
    ///
    /// ## peek
    /// peek a type without moving the cursor
    ///
    #[inline]
    pub fn peek<T: Peek>(&self, cursor: &Cursor) -> Result<bool> {
        if cursor.peek()?.is_ascii_whitespace() {
            return self.peek::<T>(cursor.fork().shift_next()?);
        }

        T::peek(cursor, self)
    }

    ///
    /// ## parse
    /// parse a type, moving the cursor forward
    ///
    #[inline]
    pub fn parse<T: Parse>(&mut self, cursor: &mut Cursor) -> Result<T> {
        if cursor.peek()?.is_ascii_whitespace() {
            return self.parse::<T>(cursor.shift_next()?);
        }

        let mut fork = cursor.fork();
        let value = T::parse(&mut fork, self)?;
        cursor.merge(fork.commit());
        Ok(value)
    }
}

impl<T: Peek> Peek for Option<T> {
    fn peek(_: &Cursor, _: &ZinqParser) -> Result<bool> {
        Ok(true)
    }
}

impl<T: Parse> Parse for Option<T> {
    fn parse(cursor: &mut Cursor, parser: &mut ZinqParser) -> Result<Self> {
        if !parser.peek::<T>(cursor).unwrap_or(false) {
            return Ok(Self::None);
        }

        Ok(Some(parser.parse::<T>(cursor)?))
    }
}

impl<T: Spanned> Spanned for Option<T> {
    fn span(&self) -> Span {
        match self {
            None => Span::default(),
            Some(v) => v.span(),
        }
    }
}

impl<T: Peek> Peek for Box<T> {
    fn peek(cursor: &Cursor, parser: &ZinqParser) -> Result<bool> {
        Ok(parser.peek::<T>(cursor).unwrap_or(false))
    }
}

impl<T: Parse> Parse for Box<T> {
    fn parse(cursor: &mut Cursor, parser: &mut ZinqParser) -> Result<Self> {
        Ok(Box::new(parser.parse::<T>(cursor)?))
    }
}

impl<T: Spanned> Spanned for Box<T> {
    fn span(&self) -> Span {
        self.as_ref().span()
    }
}
