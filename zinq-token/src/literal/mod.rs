mod byte;
mod string;

pub use byte::*;
pub use string::*;

use zinq_error::Result;
use zinq_parse::{Cursor, Parse, Parser, Peek, Span};

use crate::{Keyword, Token, TokenParser};

///
/// ## Literal
/// a literal value
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Literal {
    Byte(LByte),
    String(LString),
}

impl Literal {
    pub fn is_byte(&self) -> bool {
        match self {
            Self::Byte(_) => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            Self::String(_) => true,
            _ => false,
        }
    }
}

impl Token {
    pub fn is_literal_byte(&self) -> bool {
        if let Token::Literal(v) = &self {
            return v.is_byte();
        }

        false
    }

    pub fn is_literal_string(&self) -> bool {
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
            Self::Byte(v) => write!(f, "{}", v),
            Self::String(v) => write!(f, "{}", v),
        }
    }
}

impl Peek<TokenParser> for Literal {
    #[inline]
    fn peek(cursor: &Cursor, parser: &TokenParser) -> bool {
        todo!()
    }
}

impl Parse<TokenParser> for Literal {
    #[inline]
    fn parse(cursor: &mut Cursor, parser: &mut TokenParser) -> Result<Token> {
        todo!()
    }

    #[inline]
    fn span(&self) -> &Span {
        match self {
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
