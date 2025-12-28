use zinq_error::Result;
use zinq_parse::{Cursor, Parse, ParseError, Peek, Span};

use crate::{Literal, Token, TokenParser};

///
/// ## LInt
/// a literal int
/// ### Example
/// `0`
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LInt {
    span: Span,
}

impl LInt {
    pub fn name(&self) -> &'static str {
        "LInt"
    }
}

impl std::fmt::Display for LInt {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for LInt {
    #[inline]
    fn peek(cursor: &Cursor, parser: &TokenParser) -> Result<bool> {
        Ok(cursor.peek()?.is_ascii_digit())
    }
}

impl Parse<TokenParser> for LInt {
    #[inline]
    fn parse(cursor: &mut Cursor, parser: &mut TokenParser) -> Result<Token> {
        cursor.next_while(|b, _| b.is_ascii_digit())?;

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

impl From<LInt> for Literal {
    fn from(value: LInt) -> Self {
        Self::Int(value)
    }
}

impl From<LInt> for Token {
    fn from(value: LInt) -> Self {
        Self::Literal(Literal::Int(value))
    }
}

#[cfg(test)]
mod test {
    use zinq_error::Result;
    use zinq_parse::{Parse, Parser, Peek, Span};

    use crate::{LByte, TokenParser};

    #[test]
    fn is_int() -> Result<()> {
        let span = Span::from_bytes(b"103");
        let mut cursor = span.cursor();
        let mut parser = TokenParser;

        let token = parser.parse(&mut cursor)?;
        println!("{} => {}", token.name(), token.to_string());

        debug_assert!(token.is_literal_int());
        debug_assert_eq!(token.to_string(), "103");
        debug_assert_eq!(cursor.bytes(), b"");

        Ok(())
    }
}
