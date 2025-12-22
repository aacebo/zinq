use zinq_error::Result;
use zinq_parse::{Cursor, Parse, Parser, Peek, Span};

use crate::{Keyword, Token, TokenParser};

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
        if let Some(b) = cursor.peek() {
            return b.is_ascii_alphabetic();
        }

        false
    }
}

impl Parse<TokenParser> for Ident {
    #[inline]
    fn parse(cursor: &mut Cursor, parser: &mut TokenParser) -> Result<Token> {
        let span = cursor.next_while(|b, _| b.is_ascii_alphanumeric());

        if let Some(keyword) = Keyword::try_from_span(span) {
            return Ok(keyword.into());
        }

        Ok(Self { span: span.clone() }.into())
    }

    #[inline]
    fn span(&self) -> &Span {
        &self.span
    }
}
