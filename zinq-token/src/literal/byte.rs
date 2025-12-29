use zinq_error::Result;
use zinq_parse::{Cursor, Parse, Peek, Span};

use crate::{Literal, ToTokens, Token, TokenParser, TokenStream};

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

impl LByte {
    pub fn name(&self) -> &'static str {
        "LByte"
    }
}

impl std::fmt::Display for LByte {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for LByte {
    #[inline]
    fn peek(cursor: &Cursor, _: &TokenParser) -> Result<bool> {
        Ok(cursor.peek_n(2)? == b"b'")
    }
}

impl Parse<TokenParser> for LByte {
    #[inline]
    fn parse(cursor: &mut Cursor, _: &mut TokenParser) -> Result<Token> {
        cursor.next_while(|b, _| b != &b'\'')?.next_n(3)?;

        Ok(Self {
            span: cursor.span().clone(),
        }
        .into())
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

impl ToTokens for LByte {
    fn to_tokens(&self) -> zinq_error::Result<TokenStream> {
        Ok(Token::Literal(Literal::Byte(self.clone())).into())
    }
}

#[cfg(test)]
mod test {
    use zinq_error::Result;
    use zinq_parse::{Parser, Span};

    use crate::TokenParser;

    #[test]
    fn is_byte() -> Result<()> {
        let span = Span::from_bytes(b"b'p'");
        let mut cursor = span.cursor();
        let mut parser = TokenParser;

        let token = parser.parse(&mut cursor)?;

        debug_assert!(token.is_byte_literal());
        debug_assert_eq!(token.to_string(), "b'p'");
        debug_assert_eq!(cursor.bytes(), b"");

        Ok(())
    }
}
