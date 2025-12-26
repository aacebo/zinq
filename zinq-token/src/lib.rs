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

impl Token {
    pub fn is_group(&self) -> bool {
        match self {
            Self::Group(_) => true,
            _ => false,
        }
    }

    pub fn is_punct(&self) -> bool {
        match self {
            Self::Punct(_) => true,
            _ => false,
        }
    }

    pub fn is_literal(&self) -> bool {
        match self {
            Self::Literal(_) => true,
            _ => false,
        }
    }

    pub fn is_keyword(&self) -> bool {
        match self {
            Self::Keyword(_) => true,
            _ => false,
        }
    }

    pub fn is_ident(&self) -> bool {
        match self {
            Self::Ident(_) => true,
            _ => false,
        }
    }
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
        if cursor.last().is_ascii_whitespace() {
            return Self::parse(cursor, parser);
        }

        if Group::peek(cursor, parser) {
            return Group::parse(cursor, parser);
        }

        if Punct::peek(cursor, parser) {
            return Punct::parse(cursor, parser);
        }

        if Literal::peek(cursor, parser) {
            return Literal::parse(cursor, parser);
        }

        Ident::parse(cursor, parser)
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
