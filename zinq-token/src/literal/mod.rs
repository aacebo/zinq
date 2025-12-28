mod byte;
mod int;
mod string;

pub use byte::*;
pub use int::*;
pub use string::*;

use zinq_error::{NOT_FOUND, Result};
use zinq_parse::{Cursor, Parse, Parser, Peek, Span};

use crate::{Token, TokenMismatchError, TokenParser};

///
/// ## Literal
/// a literal value
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Literal {
    Int(LInt),
    Byte(LByte),
    String(LString),
}

impl Literal {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Int(v) => v.name(),
            Self::Byte(v) => v.name(),
            Self::String(v) => v.name(),
        }
    }

    pub fn is_int(&self) -> bool {
        match self {
            Self::Int(_) => true,
            _ => false,
        }
    }

    pub fn try_to_int(&self) -> Result<&LInt> {
        match self {
            Self::Int(v) => Ok(v),
            other => {
                Err(
                    TokenMismatchError::from_types("LInt", other.name(), other.span().clone())
                        .into(),
                )
            }
        }
    }

    pub fn is_byte(&self) -> bool {
        match self {
            Self::Byte(_) => true,
            _ => false,
        }
    }

    pub fn try_to_byte(&self) -> Result<&LByte> {
        match self {
            Self::Byte(v) => Ok(v),
            other => {
                Err(
                    TokenMismatchError::from_types("LByte", other.name(), other.span().clone())
                        .into(),
                )
            }
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            Self::String(_) => true,
            _ => false,
        }
    }

    pub fn try_to_string(&self) -> Result<&LString> {
        match self {
            Self::String(v) => Ok(v),
            other => {
                Err(
                    TokenMismatchError::from_types("LString", other.name(), other.span().clone())
                        .into(),
                )
            }
        }
    }
}

impl Token {
    pub fn is_int_literal(&self) -> bool {
        if let Token::Literal(v) = &self {
            return v.is_int();
        }

        false
    }

    pub fn is_byte_literal(&self) -> bool {
        if let Token::Literal(v) = &self {
            return v.is_byte();
        }

        false
    }

    pub fn is_string_literal(&self) -> bool {
        if let Token::Literal(v) = &self {
            return v.is_string();
        }

        false
    }
}

impl std::fmt::Display for Literal {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(v) => write!(f, "{}", v),
            Self::Byte(v) => write!(f, "{}", v),
            Self::String(v) => write!(f, "{}", v),
        }
    }
}

impl Peek<TokenParser> for Literal {
    #[inline]
    fn peek(cursor: &Cursor, parser: &TokenParser) -> Result<bool> {
        if parser.peek_as::<LInt>(cursor)? {
            return Ok(true);
        }

        if parser.peek_as::<LByte>(cursor)? {
            return Ok(true);
        }

        parser.peek_as::<LString>(cursor)
    }
}

impl Parse<TokenParser> for Literal {
    #[inline]
    fn parse(cursor: &mut Cursor, parser: &mut TokenParser) -> Result<Token> {
        if parser.peek_as::<LInt>(cursor)? {
            return parser.parse_as::<LInt>(cursor);
        }

        if parser.peek_as::<LByte>(cursor)? {
            return parser.parse_as::<LByte>(cursor);
        }

        if parser.peek_as::<LString>(cursor)? {
            return parser.parse_as::<LString>(cursor);
        }

        Err(cursor.error(NOT_FOUND, "not found"))
    }

    #[inline]
    fn span(&self) -> &Span {
        match self {
            Self::Int(v) => v.span(),
            Self::Byte(v) => v.span(),
            Self::String(v) => v.span(),
        }
    }
}

impl From<Literal> for Token {
    #[inline]
    fn from(value: Literal) -> Self {
        Self::Literal(value)
    }
}
