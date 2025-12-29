mod close;
mod open;

pub use close::*;
pub use open::*;

use zinq_parse::{Parse, Parser, Peek};

use crate::{ToTokens, Token, TokenParser};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Delim {
    Open(OpenDelim),
    Close(CloseDelim),
}

impl Delim {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Close(v) => v.name(),
            Self::Open(v) => v.name(),
        }
    }

    pub fn is_open(&self) -> bool {
        match self {
            Self::Open(_) => true,
            _ => false,
        }
    }

    pub fn is_close(&self) -> bool {
        match self {
            Self::Close(_) => true,
            _ => false,
        }
    }
}

impl Token {
    pub fn is_open_delim(&self) -> bool {
        match self {
            Self::Delim(v) => v.is_open(),
            _ => false,
        }
    }

    pub fn is_close_delim(&self) -> bool {
        match self {
            Self::Delim(v) => v.is_close(),
            _ => false,
        }
    }
}

impl std::fmt::Display for Delim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Open(v) => write!(f, "{}", v),
            Self::Close(v) => write!(f, "{}", v),
        }
    }
}

impl Peek<TokenParser> for Delim {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        if parser.peek_as::<OpenDelim>(cursor)? {
            return Ok(true);
        }

        if parser.peek_as::<CloseDelim>(cursor)? {
            return Ok(true);
        }

        Ok(false)
    }
}

impl Parse<TokenParser> for Delim {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<<TokenParser as Parser>::Item> {
        if parser.peek_as::<OpenDelim>(cursor)? {
            return parser.parse_as::<OpenDelim>(cursor);
        }

        if parser.peek_as::<CloseDelim>(cursor)? {
            return parser.parse_as::<CloseDelim>(cursor);
        }

        Err(cursor.error(
            zinq_error::NOT_FOUND,
            &format!("unknown tokens '{}'", cursor),
        ))
    }

    fn span(&self) -> &zinq_parse::Span {
        match self {
            Self::Open(v) => v.span(),
            Self::Close(v) => v.span(),
        }
    }
}

impl ToTokens for Delim {
    fn to_tokens(&self) -> zinq_error::Result<crate::TokenStream> {
        Ok(Token::Delim(self.clone()).into())
    }
}
