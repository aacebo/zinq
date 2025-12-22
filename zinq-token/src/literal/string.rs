use zinq_error::Result;
use zinq_parse::{Cursor, Parse, Peek, Span};

use crate::{Literal, Token, TokenParser};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LString {
    span: Span,
}

impl std::fmt::Display for LString {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for LString {
    #[inline]
    fn peek(cursor: &Cursor, parser: &TokenParser) -> bool {
        todo!()
    }
}

impl Parse<TokenParser> for LString {
    #[inline]
    fn parse(cursor: &mut Cursor, parser: &mut TokenParser) -> Result<Token> {
        todo!()
    }

    #[inline]
    fn span(&self) -> &Span {
        &self.span
    }
}

impl From<LString> for Literal {
    fn from(value: LString) -> Self {
        Self::String(value)
    }
}

impl From<LString> for Token {
    fn from(value: LString) -> Self {
        Self::Literal(Literal::String(value))
    }
}
