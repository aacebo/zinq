mod delimiter;
mod enclosed;
mod error;
mod ident;
mod keyword;
mod literal;
pub mod macros;
mod prefixed;
mod punct;
mod punctuated;
mod stream;
mod suffixed;

pub use delimiter::*;
pub use enclosed::*;
pub use error::*;
pub use ident::*;
pub use keyword::*;
pub use literal::*;
pub use prefixed::*;
pub use punct::*;
pub use punctuated::*;
pub use stream::*;
pub use suffixed::*;

use zinq_error::Result;
use zinq_parse::{Cursor, Parse, Parser, Peek, Span};

///
/// ## ToTokens
/// when implemented makes any type
/// token stream compatible
///
pub trait ToTokens {
    fn to_tokens(&self) -> Result<TokenStream>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Punct(Punct),
    Literal(Literal),
    Keyword(Keyword),
    Ident(Ident),
    Delim(Delim),
}

impl Token {
    pub fn is_punct(&self) -> bool {
        match self {
            Self::Punct(_) => true,
            _ => false,
        }
    }

    pub fn try_to_punct(&self) -> Result<&Punct> {
        match self {
            Self::Punct(v) => Ok(v),
            other => {
                Err(
                    TokenMismatchError::from_types("Punct", other.name(), other.span().clone())
                        .into(),
                )
            }
        }
    }

    pub fn is_literal(&self) -> bool {
        match self {
            Self::Literal(_) => true,
            _ => false,
        }
    }

    pub fn try_to_literal(&self) -> Result<&Literal> {
        match self {
            Self::Literal(v) => Ok(v),
            other => {
                Err(
                    TokenMismatchError::from_types("Literal", other.name(), other.span().clone())
                        .into(),
                )
            }
        }
    }

    pub fn is_keyword(&self) -> bool {
        match self {
            Self::Keyword(_) => true,
            _ => false,
        }
    }

    pub fn try_to_keyword(&self) -> Result<&Keyword> {
        match self {
            Self::Keyword(v) => Ok(v),
            other => {
                Err(
                    TokenMismatchError::from_types("Keyword", other.name(), other.span().clone())
                        .into(),
                )
            }
        }
    }

    pub fn is_ident(&self) -> bool {
        match self {
            Self::Ident(_) => true,
            _ => false,
        }
    }

    pub fn try_to_ident(&self) -> Result<&Ident> {
        match self {
            Self::Ident(v) => Ok(v),
            other => {
                Err(
                    TokenMismatchError::from_types("Ident", other.name(), other.span().clone())
                        .into(),
                )
            }
        }
    }

    pub fn is_delim(&self) -> bool {
        match self {
            Self::Delim(_) => true,
            _ => false,
        }
    }

    pub fn try_to_delim(&self) -> Result<&Delim> {
        match self {
            Self::Delim(v) => Ok(v),
            other => {
                Err(
                    TokenMismatchError::from_types("Delim", other.name(), other.span().clone())
                        .into(),
                )
            }
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Ident(v) => v.name(),
            Self::Keyword(v) => v.name(),
            Self::Literal(v) => v.name(),
            Self::Punct(v) => v.name(),
            Self::Delim(v) => v.name(),
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ident(v) => write!(f, "{}", v),
            Self::Keyword(v) => write!(f, "{}", v),
            Self::Literal(v) => write!(f, "{}", v),
            Self::Punct(v) => write!(f, "{}", v),
            Self::Delim(v) => write!(f, "{}", v),
        }
    }
}

impl Peek for Token {
    #[allow(unused)]
    #[inline]
    fn peek(cursor: &Cursor, parser: &zinq_parse::ZinqParser) -> Result<bool> {
        Ok(true)
    }
}

impl Parse for Token {
    #[inline]
    fn parse(cursor: &mut Cursor, parser: &mut zinq_parse::ZinqParser) -> Result<Self> {
        if cursor.peek()?.is_ascii_whitespace() {
            return parser.parse(cursor.shift_next()?);
        }

        if parser.peek::<Punct>(cursor)? {
            return Ok(parser.parse::<Punct>(cursor)?.into());
        }

        if parser.peek::<Delim>(cursor)? {
            return Ok(parser.parse::<Delim>(cursor)?.into());
        }

        if parser.peek::<Literal>(cursor)? {
            return Ok(parser.parse::<Literal>(cursor)?.into());
        }

        if parser.peek::<Keyword>(cursor)? {
            return Ok(parser.parse::<Keyword>(cursor)?.into());
        }

        Ok(parser.parse::<Ident>(cursor)?.into())
    }

    #[inline]
    fn span(&self) -> &Span {
        match self {
            Self::Punct(v) => v.span(),
            Self::Literal(v) => v.span(),
            Self::Keyword(v) => v.span(),
            Self::Ident(v) => v.span(),
            Self::Delim(v) => v.span(),
        }
    }
}

impl ToTokens for Token {
    fn to_tokens(&self) -> Result<TokenStream> {
        Ok(self.clone().into())
    }
}

#[cfg(test)]
mod test {
    use zinq_error::Result;
    use zinq_parse::{Parser, Span};

    use crate::zinq_parse::ZinqParser;

    #[test]
    fn is_string_assignment() -> Result<()> {
        let span = Span::from_bytes(b"let test: string = \"test\";");
        let mut cursor = span.cursor();
        let mut parser = zinq_parse::ZinqParser;
        let mut token = parser.parse(&mut cursor)?;

        debug_assert!(token.is_keyword());
        debug_assert_eq!(token.to_string(), "let");

        token = parser.parse(&mut cursor)?;

        debug_assert!(token.is_ident());
        debug_assert_eq!(token.to_string(), "test");

        token = parser.parse(&mut cursor)?;

        debug_assert!(token.is_colon());
        debug_assert_eq!(token.to_string(), ":");

        token = parser.parse(&mut cursor)?;

        debug_assert!(token.is_ident());
        debug_assert_eq!(token.to_string(), "string");

        token = parser.parse(&mut cursor)?;

        debug_assert!(token.is_eq());
        debug_assert_eq!(token.to_string(), "=");

        token = parser.parse(&mut cursor)?;

        debug_assert!(token.is_string_literal());
        debug_assert_eq!(token.to_string(), "\"test\"");

        token = parser.parse(&mut cursor)?;

        debug_assert!(token.is_semi_colon());
        debug_assert_eq!(token.to_string(), ";");

        Ok(())
    }

    #[test]
    fn is_int_assignment() -> Result<()> {
        let span = Span::from_bytes(b"let test: int = 121u16;");
        let mut cursor = span.cursor();
        let mut parser = zinq_parse::ZinqParser;
        let mut token = parser.parse(&mut cursor)?;

        debug_assert!(token.is_keyword());
        debug_assert_eq!(token.to_string(), "let");

        token = parser.parse(&mut cursor)?;

        debug_assert!(token.is_ident());
        debug_assert_eq!(token.to_string(), "test");

        token = parser.parse(&mut cursor)?;

        debug_assert!(token.is_colon());
        debug_assert_eq!(token.to_string(), ":");

        token = parser.parse(&mut cursor)?;

        debug_assert!(token.is_ident());
        debug_assert_eq!(token.to_string(), "int");

        token = parser.parse(&mut cursor)?;

        debug_assert!(token.is_eq());
        debug_assert_eq!(token.to_string(), "=");

        token = parser.parse(&mut cursor)?;

        debug_assert!(token.is_u16_literal());
        debug_assert_eq!(token.to_string(), "121u16");

        token = parser.parse(&mut cursor)?;

        debug_assert!(token.is_semi_colon());
        debug_assert_eq!(token.to_string(), ";");

        Ok(())
    }
}
