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
use zinq_parse::{Cursor, Parse, Parser, Peek, Span};

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

    pub fn name(&self) -> &'static str {
        match self {
            Self::Group(v) => v.name(),
            Self::Ident(v) => v.name(),
            Self::Keyword(v) => v.name(),
            Self::Literal(v) => v.name(),
            Self::Punct(v) => v.name(),
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
    fn peek(cursor: &Cursor, parser: &TokenParser) -> Result<bool> {
        Ok(true)
    }
}

impl Parse<TokenParser> for Token {
    #[inline]
    fn parse(cursor: &mut Cursor, parser: &mut TokenParser) -> Result<Self> {
        if cursor.peek()?.is_ascii_whitespace() {
            return parser.parse(cursor.shift_next()?);
        }

        if Group::peek(cursor, parser)? {
            return Group::parse(cursor, parser);
        }

        if Punct::peek(cursor, parser)? {
            return parser.parse_as::<Punct>(cursor);
        }

        if Literal::peek(cursor, parser)? {
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

#[cfg(test)]
mod test {
    use zinq_error::Result;
    use zinq_parse::{Parser, Span};

    use crate::{Ident, TokenParser};

    #[test]
    fn should_parse_assignment() -> Result<()> {
        let span = Span::from_bytes(b"let test: string = \"test\";");
        let mut cursor = span.cursor();
        let mut parser = TokenParser;
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

        debug_assert!(token.is_literal_string());
        debug_assert_eq!(token.to_string(), "\"test\"");

        token = parser.parse(&mut cursor)?;

        debug_assert!(token.is_semi_colon());
        debug_assert_eq!(token.to_string(), ";");

        Ok(())
    }
}
