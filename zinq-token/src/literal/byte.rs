use zinq_error::Result;
use zinq_parse::{Cursor, Parse, ParseError, Peek, Span};

use crate::{Literal, Token, TokenParser};

///
/// ## LByte
/// a literal byte
/// ### Example
/// `b'a'`
/// 
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LByte {
    span: Span,
}

impl std::fmt::Display for LByte {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for LByte {
    #[inline]
    fn peek(cursor: &Cursor, parser: &TokenParser) -> bool {
        cursor.peek_n(2) == b"b'"
    }
}

impl Parse<TokenParser> for LByte {
    #[inline]
    fn parse(cursor: &mut Cursor, parser: &mut TokenParser) -> Result<Token> {
        let span = cursor.next_while(|_, span| !span.bytes().ends_with(b"'"));

        if !span.bytes().ends_with(b"'") {
            //Err(cursor.error(""))
        }
    }

    #[inline]
    fn span(&self) -> &Span {
        &self.span
    }
}

impl From<LByte> for Literal {
    fn from(value: LByte) -> Self {
        Self::Byte(value)
    }
}

impl From<LByte> for Token {
    fn from(value: LByte) -> Self {
        Self::Literal(Literal::Byte(value))
    }
}
