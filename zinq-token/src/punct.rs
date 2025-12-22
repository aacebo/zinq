use zinq_error::Result;
use zinq_parse::{Cursor, Parse, Parser, Peek, Span};

use crate::{Keyword, Token, TokenParser};

///
/// ## Punct
/// a single punctuation character
/// ### Examples
/// - +
/// - -
/// - =
/// - ,
/// - <
/// - >
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Punct {
    span: Span,
}

impl From<Punct> for Token {
    #[inline]
    fn from(value: Punct) -> Self {
        Self::Punct(value)
    }
}

impl std::fmt::Display for Punct {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for Punct {
    #[inline]
    fn peek(cursor: &Cursor, parser: &TokenParser) -> bool {
        todo!()
    }
}

impl Parse<TokenParser> for Punct {
    #[inline]
    fn parse(cursor: &mut Cursor, parser: &mut TokenParser) -> Result<Token> {
        todo!()
    }

    #[inline]
    fn span(&self) -> &Span {
        &self.span
    }
}
