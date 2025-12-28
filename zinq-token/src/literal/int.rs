use zinq_error::Result;
use zinq_parse::{Cursor, Parse, Peek, Span};

use crate::{Literal, Token, TokenParser};

///
/// ## LInt
/// a literal int
/// ### Example
/// `0` or `0u8` or `0i8` etc
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LInt {
    span: Span,
}

impl LInt {
    pub fn name(&self) -> &'static str {
        "LInt"
    }

    pub fn is_u8(&self) -> bool {
        self.span.bytes().ends_with(b"u8")
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
    fn peek(cursor: &Cursor, _: &TokenParser) -> Result<bool> {
        Ok(cursor.peek()?.is_ascii_digit())
    }
}

impl Parse<TokenParser> for LInt {
    #[inline]
    fn parse(cursor: &mut Cursor, _: &mut TokenParser) -> Result<Token> {
        cursor.next_while(|b, _| b.is_ascii_digit())?;

        if cursor.peek_n(2).unwrap_or(&[]) == b"u8" || cursor.peek_n(2).unwrap_or(&[]) == b"i8" {
            cursor.next_n(2)?;

            return Ok(Self {
                span: cursor.span().clone(),
            }
            .into());
        }

        if cursor.peek_n(3).unwrap_or(&[]) == b"u16"
            || cursor.peek_n(3).unwrap_or(&[]) == b"i16"
            || cursor.peek_n(3).unwrap_or(&[]) == b"u32"
            || cursor.peek_n(3).unwrap_or(&[]) == b"i32"
            || cursor.peek_n(3).unwrap_or(&[]) == b"u64"
            || cursor.peek_n(3).unwrap_or(&[]) == b"i64"
        {
            cursor.next_n(3)?;

            return Ok(Self {
                span: cursor.span().clone(),
            }
            .into());
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
    use zinq_parse::{Parser, Span};

    use crate::TokenParser;

    #[test]
    fn is_int() -> Result<()> {
        let span = Span::from_bytes(b"103");
        let mut cursor = span.cursor();
        let mut parser = TokenParser;

        let mut token = parser.parse(&mut cursor)?;
        println!("{} => {}", token.name(), token.to_string());

        debug_assert!(token.is_literal_int());
        debug_assert_eq!(token.to_string(), "103");
        debug_assert_eq!(cursor.bytes(), b"");

        cursor = Span::from_bytes(b"103u8").cursor();
        token = parser.parse(&mut cursor)?;

        debug_assert!(token.is_literal_int());
        debug_assert_eq!(token.to_string(), "103u8");

        Ok(())
    }
}
