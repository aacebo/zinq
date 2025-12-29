use zinq_error::Result;
use zinq_parse::{Cursor, Parse, Peek, Span};

use crate::{Literal, ToTokens, Token, TokenParser, TokenStream};

///
/// ## LBool
/// a literal boolean
/// ### Example
/// `true` or `false`
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LBool {
    span: Span,
}

impl LBool {
    pub fn name(&self) -> &'static str {
        "LBool"
    }
}

impl std::fmt::Display for LBool {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for LBool {
    #[inline]
    fn peek(cursor: &Cursor, _: &TokenParser) -> Result<bool> {
        Ok(cursor.peek_n(4).unwrap_or(&[]) == b"true"
            || cursor.peek_n(5).unwrap_or(&[]) == b"false")
    }
}

impl Parse<TokenParser> for LBool {
    #[inline]
    fn parse(cursor: &mut Cursor, _: &mut TokenParser) -> Result<Token> {
        if cursor.peek_n(4).unwrap_or(&[]) == b"true" {
            cursor.next_n(4)?;
        }

        if cursor.peek_n(5).unwrap_or(&[]) == b"false" {
            cursor.next_n(5)?;
        }

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

impl From<LBool> for Literal {
    fn from(value: LBool) -> Self {
        Self::Bool(value)
    }
}

impl From<LBool> for Token {
    fn from(value: LBool) -> Self {
        Self::Literal(Literal::Bool(value))
    }
}

impl ToTokens for LBool {
    fn to_tokens(&self) -> zinq_error::Result<TokenStream> {
        Ok(Token::Literal(Literal::Bool(self.clone())).into())
    }
}

#[cfg(test)]
mod test {
    use zinq_error::Result;
    use zinq_parse::{Parser, Span};

    use crate::TokenParser;

    #[test]
    fn is_bool() -> Result<()> {
        let span = Span::from_bytes(b"false");
        let mut cursor = span.cursor();
        let mut parser = TokenParser;

        let token = parser.parse(&mut cursor)?;

        debug_assert!(token.is_bool_literal());
        debug_assert_eq!(token.to_string(), "false");

        Ok(())
    }
}
