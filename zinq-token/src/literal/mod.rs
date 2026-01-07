mod bool;
mod byte;
mod float;
mod int;
mod string;

pub use bool::*;
pub use byte::*;
pub use float::*;
pub use int::*;
pub use string::*;

use zinq_error::{NOT_FOUND, Result};
use zinq_parse::{Cursor, Parse, Peek, Span, Spanned};

use crate::{ToTokens, Token, TokenMismatchError, TokenStream};

///
/// ## Literal
/// a literal value
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Literal {
    Int(LInt),
    Float(LFloat),
    Byte(LByte),
    Bool(LBool),
    String(LString),
}

impl Literal {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Int(v) => v.name(),
            Self::Float(v) => v.name(),
            Self::Byte(v) => v.name(),
            Self::Bool(v) => v.name(),
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

    pub fn is_float(&self) -> bool {
        match self {
            Self::Float(_) => true,
            _ => false,
        }
    }

    pub fn try_to_float(&self) -> Result<&LFloat> {
        match self {
            Self::Float(v) => Ok(v),
            other => {
                Err(
                    TokenMismatchError::from_types("LFloat", other.name(), other.span().clone())
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

    pub fn is_bool(&self) -> bool {
        match self {
            Self::Bool(_) => true,
            _ => false,
        }
    }

    pub fn try_to_bool(&self) -> Result<&LBool> {
        match self {
            Self::Bool(v) => Ok(v),
            other => {
                Err(
                    TokenMismatchError::from_types("LBool", other.name(), other.span().clone())
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

    pub fn is_float_literal(&self) -> bool {
        if let Token::Literal(v) = &self {
            return v.is_float();
        }

        false
    }

    pub fn is_byte_literal(&self) -> bool {
        if let Token::Literal(v) = &self {
            return v.is_byte();
        }

        false
    }

    pub fn is_bool_literal(&self) -> bool {
        if let Token::Literal(v) = &self {
            return v.is_bool();
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
            Self::Float(v) => write!(f, "{}", v),
            Self::Byte(v) => write!(f, "{}", v),
            Self::Bool(v) => write!(f, "{}", v),
            Self::String(v) => write!(f, "{}", v),
        }
    }
}

impl Peek for Literal {
    #[inline]
    fn peek(cursor: &Cursor, parser: &zinq_parse::ZinqParser) -> Result<bool> {
        if parser.peek::<LFloat>(cursor)? {
            return Ok(true);
        }

        if parser.peek::<LInt>(cursor)? {
            return Ok(true);
        }

        if parser.peek::<LByte>(cursor)? {
            return Ok(true);
        }

        if parser.peek::<LBool>(cursor)? {
            return Ok(true);
        }

        parser.peek::<LString>(cursor)
    }
}

impl Parse for Literal {
    #[inline]
    fn parse(cursor: &mut Cursor, parser: &mut zinq_parse::ZinqParser) -> Result<Self> {
        if parser.peek::<LFloat>(cursor)? {
            return Ok(parser.parse::<LFloat>(cursor)?.into());
        }

        if parser.peek::<LInt>(cursor)? {
            return Ok(parser.parse::<LInt>(cursor)?.into());
        }

        if parser.peek::<LByte>(cursor)? {
            return Ok(parser.parse::<LByte>(cursor)?.into());
        }

        if parser.peek::<LString>(cursor)? {
            return Ok(parser.parse::<LString>(cursor)?.into());
        }

        if parser.peek::<LBool>(cursor)? {
            return Ok(parser.parse::<LBool>(cursor)?.into());
        }

        Err(cursor.error(NOT_FOUND, "not found"))
    }
}

impl Spanned for Literal {
    fn span(&self) -> Span {
        match self {
            Self::Int(v) => v.span(),
            Self::Float(v) => v.span(),
            Self::Byte(v) => v.span(),
            Self::Bool(v) => v.span(),
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

impl ToTokens for Literal {
    fn to_tokens(&self) -> zinq_error::Result<TokenStream> {
        Ok(Token::Literal(self.clone()).into())
    }
}
