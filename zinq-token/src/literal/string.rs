use zinq_error::Result;
use zinq_parse::{Cursor, Parse, Peek, Span, Spanned};

use crate::{Literal, ToTokens, Token, TokenStream};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LString {
    span: Span,
}

impl LString {
    pub fn name(&self) -> &'static str {
        "LString"
    }
}

impl std::fmt::Display for LString {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek for LString {
    #[inline]
    fn peek(cursor: &Cursor, _: &zinq_parse::ZinqParser) -> Result<bool> {
        Ok(cursor.peek()? == &b'"')
    }
}

impl Parse for LString {
    #[inline]
    fn parse(cursor: &mut Cursor, _: &mut zinq_parse::ZinqParser) -> Result<Self> {
        cursor.next()?.next_while(|next, _| next != &b'"')?.next()?;

        Ok(Self {
            span: cursor.span().clone(),
        })
    }
}

impl Spanned for LString {
    fn span(&self) -> Span {
        self.span.clone()
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

impl ToTokens for LString {
    fn to_tokens(&self) -> zinq_error::Result<TokenStream> {
        Ok(Token::Literal(Literal::String(self.clone())).into())
    }
}

#[cfg(test)]
mod tests {
    use zinq_error::Result;
    use zinq_parse::Span;

    use crate::{LString, Token};

    #[test]
    fn is_string() -> Result<()> {
        let span = Span::from_bytes(b"\"test\"");
        let mut cursor = span.cursor();
        let mut parser = zinq_parse::ZinqParser;

        debug_assert!(parser.peek::<LString>(&cursor)?);

        let token = parser.parse::<Token>(&mut cursor)?;

        debug_assert!(token.is_string_literal());
        debug_assert_eq!(token.to_string(), "\"test\"");
        debug_assert_eq!(cursor.bytes(), b"");

        Ok(())
    }
}
