use zinq_error::Result;
use zinq_parse::{Cursor, Parse, Parser, Peek, Span};

use crate::TokenParser;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ident {
    span: Span,
}

impl From<Ident> for crate::Token {
    #[inline]
    fn from(value: Ident) -> Self {
        Self::Ident(value)
    }
}

impl std::fmt::Display for Ident {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for Ident {
    #[inline]
    fn peek(cursor: &Cursor, parser: &TokenParser) -> bool {
        true
    }
}

impl Parse<TokenParser> for Ident {
    #[inline]
    fn parse(cursor: &mut Cursor, parser: &mut TokenParser) -> Result<Self> {
        todo!()
    }

    #[inline]
    fn span(&self) -> &Span {
        &self.span
    }
}
