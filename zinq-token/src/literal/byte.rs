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
        cursor.last() == &b'b' && cursor.peek().unwrap_or(&0) == &b'\''
    }
}

impl Parse<TokenParser> for LByte {
    #[inline]
    fn parse(cursor: &mut Cursor, parser: &mut TokenParser) -> Result<Token> {
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

#[cfg(test)]
mod test {
    use zinq_parse::{Parse, Parser, Peek, Span};

    use crate::{LByte, TokenParser};

    #[test]
    fn should_parse_byte() {
        let span = Span::from_bytes(b"b'p'");
        let mut cursor = span.cursor();
        let mut parser = TokenParser;

        debug_assert!(LByte::peek(&mut cursor, &mut parser));

        let token = parser
            .parse_as::<LByte>(&mut cursor)
            .expect("should not error");

        debug_assert!(token.is_literal_byte());
        debug_assert_eq!(token.to_string(), "b'p'");
        debug_assert_eq!(cursor.bytes(), b"'");
    }
}
