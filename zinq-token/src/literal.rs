use zinq_error::Result;
use zinq_parse::{Cursor, Parse, Parser, Peek, Span};

use crate::{Keyword, Token, TokenParser};

///
/// ## Literal
/// a literal value
/// ### Examples
/// - 123
/// - true/false
/// - "test"
/// - 'h'
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Literal {
    span: Span,
}

impl From<Literal> for Token {
    #[inline]
    fn from(value: Literal) -> Self {
        Self::Literal(value)
    }
}

impl std::fmt::Display for Literal {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
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
        &self.span
    }
}
