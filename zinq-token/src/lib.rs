#![allow(unused)]

mod group;
mod ident;
mod keyword;
mod literal;
mod parser;
mod punct;
mod stream;

pub use group::*;
pub use ident::*;
pub use keyword::*;
pub use literal::*;
pub use parser::*;
pub use punct::*;
pub use stream::*;

use zinq_error::Result;
use zinq_parse::{Cursor, Parse, Peek, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Group(Group),
    Punct(Punct),
    Literal(Literal),
    Keyword(Keyword),
    Ident(Ident),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Group(v) => write!(f, "{}", v),
            Self::Ident(v) => write!(f, "{}", v),
            Self::Keyword(v) => write!(f, "{}", v),
            Self::Literal(v) => write!(f, "{}", v),
            Self::Punct(v) => write!(f, "{}", v),
        }
    }
}

impl Peek<TokenParser> for Token {
    #[inline]
    fn peek(cursor: &Cursor, parser: &TokenParser) -> bool {
        true
    }
}

impl Parse<TokenParser> for Token {
    #[inline]
    fn parse(cursor: &mut Cursor, parser: &mut TokenParser) -> Result<Self> {
        match Ident::parse(cursor, parser) {
            Err(err) => Err(err),
            Ok(v) => Ok(v.into()),
        }
    }

    #[inline]
    fn span(&self) -> &Span {
        match self {
            Self::Group(v) => v.span(),
            Self::Punct(v) => v.span(),
            Self::Literal(v) => v.span(),
            Self::Keyword(v) => v.span(),
            Self::Ident(v) => v.span(),
        }
    }
}
