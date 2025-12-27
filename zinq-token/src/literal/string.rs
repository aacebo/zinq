use zinq_error::Result;
use zinq_parse::{Cursor, Parse, Peek, Span};

use crate::{Literal, Token, TokenParser};

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

impl Peek<TokenParser> for LString {
    #[inline]
    fn peek(cursor: &Cursor, parser: &TokenParser) -> Result<bool> {
        Ok(cursor.peek()? == &b'"')
    }
}

impl Parse<TokenParser> for LString {
    #[inline]
    fn parse(cursor: &mut Cursor, parser: &mut TokenParser) -> Result<Token> {
        cursor.next()?.next_while(|next, _| next != &b'"')?.next()?;

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

#[cfg(test)]
mod test {
    use zinq_error::Result;
    use zinq_parse::{Parse, Parser, Peek, Span};

    use crate::{LString, TokenParser};

    #[test]
    fn is_string() -> Result<()> {
        let span = Span::from_bytes(b"\"test\"");
        let mut cursor = span.cursor();
        let mut parser = TokenParser;

        debug_assert!(parser.peek_as::<LString>(&cursor)?);

        let token = parser.parse_as::<LString>(&mut cursor)?;

        debug_assert!(token.is_literal_string());
        debug_assert_eq!(token.to_string(), "\"test\"");
        debug_assert_eq!(cursor.bytes(), b"");

        Ok(())
    }
}
