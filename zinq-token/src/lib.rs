mod delimiter;
mod ident;
mod keyword;
mod literal;
mod parser;
mod punct;
mod stream;

pub use delimiter::*;
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

    pub fn is_delim(&self) -> bool {
        match self {
            Self::Delim(_) => true,
            _ => false,
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

impl Peek<TokenParser> for Token {
    #[allow(unused)]
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

        if parser.peek_as::<Punct>(cursor)? {
            return parser.parse_as::<Punct>(cursor);
        }

        if parser.peek_as::<Delim>(cursor)? {
            return parser.parse_as::<Delim>(cursor);
        }

        if parser.peek_as::<Literal>(cursor)? {
            return parser.parse_as::<Literal>(cursor);
        }

        parser.parse_as::<Ident>(cursor)
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

#[cfg(test)]
mod test {
    use zinq_error::Result;
    use zinq_parse::{Parser, Span};

    use crate::TokenParser;

    #[test]
    fn is_assignment() -> Result<()> {
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
