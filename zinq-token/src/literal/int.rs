use zinq_error::Result;
use zinq_parse::{Cursor, Parse, Peek, Span};

use crate::{Literal, ToTokens, Token, TokenStream, zinq_parse::ZinqParser};

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

    pub fn is_i8(&self) -> bool {
        self.span.bytes().ends_with(b"i8")
    }

    pub fn is_u16(&self) -> bool {
        self.span.bytes().ends_with(b"u16")
    }

    pub fn is_i16(&self) -> bool {
        self.span.bytes().ends_with(b"i16")
    }

    pub fn is_u32(&self) -> bool {
        self.span.bytes().ends_with(b"u32")
    }

    pub fn is_i32(&self) -> bool {
        self.span.bytes().ends_with(b"i32")
    }

    pub fn is_u64(&self) -> bool {
        self.span.bytes().ends_with(b"u64")
    }

    pub fn is_i64(&self) -> bool {
        self.span.bytes().ends_with(b"i64")
    }

    pub fn digits(&self) -> &[u8] {
        if self.is_u8() || self.is_i8() {
            return &self.span.bytes()[0..self.span.len() - 2];
        }

        if self.is_u16()
            || self.is_i16()
            || self.is_u32()
            || self.is_i32()
            || self.is_u64()
            || self.is_i64()
        {
            return &self.span.bytes()[0..self.span.len() - 3];
        }

        self.span.bytes()
    }

    pub fn suffix(&self) -> &[u8] {
        if self.is_u8() || self.is_i8() {
            return &self.span.bytes()[self.span.len() - 2..self.span.len()];
        }

        if self.is_u16()
            || self.is_i16()
            || self.is_u32()
            || self.is_i32()
            || self.is_u64()
            || self.is_i64()
        {
            return &self.span.bytes()[self.span.len() - 3..self.span.len()];
        }

        &[]
    }
}

impl Literal {
    pub fn is_i8(&self) -> bool {
        match self {
            Self::Int(v) => v.is_i8(),
            _ => false,
        }
    }

    pub fn is_u8(&self) -> bool {
        match self {
            Self::Int(v) => v.is_u8(),
            _ => false,
        }
    }

    pub fn is_i16(&self) -> bool {
        match self {
            Self::Int(v) => v.is_i16(),
            _ => false,
        }
    }

    pub fn is_u16(&self) -> bool {
        match self {
            Self::Int(v) => v.is_u16(),
            _ => false,
        }
    }

    pub fn is_i32(&self) -> bool {
        match self {
            Self::Int(v) => v.is_i32(),
            _ => false,
        }
    }

    pub fn is_u32(&self) -> bool {
        match self {
            Self::Int(v) => v.is_u32(),
            _ => false,
        }
    }

    pub fn is_i64(&self) -> bool {
        match self {
            Self::Int(v) => v.is_i64(),
            _ => false,
        }
    }

    pub fn is_u64(&self) -> bool {
        match self {
            Self::Int(v) => v.is_u64(),
            _ => false,
        }
    }
}

impl Token {
    pub fn is_u8_literal(&self) -> bool {
        self.is_literal() && self.try_to_literal().expect("must be literal").is_u8()
    }

    pub fn is_i8_literal(&self) -> bool {
        self.is_literal() && self.try_to_literal().expect("must be literal").is_i8()
    }

    pub fn is_u16_literal(&self) -> bool {
        self.is_literal() && self.try_to_literal().expect("must be literal").is_u16()
    }

    pub fn is_i16_literal(&self) -> bool {
        self.is_literal() && self.try_to_literal().expect("must be literal").is_i16()
    }

    pub fn is_u32_literal(&self) -> bool {
        self.is_literal() && self.try_to_literal().expect("must be literal").is_u32()
    }

    pub fn is_i32_literal(&self) -> bool {
        self.is_literal() && self.try_to_literal().expect("must be literal").is_i32()
    }

    pub fn is_u64_literal(&self) -> bool {
        self.is_literal() && self.try_to_literal().expect("must be literal").is_u64()
    }

    pub fn is_i64_literal(&self) -> bool {
        self.is_literal() && self.try_to_literal().expect("must be literal").is_i64()
    }
}

impl std::fmt::Display for LInt {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek for LInt {
    #[inline]
    fn peek(cursor: &Cursor, _: &zinq_parse::ZinqParser) -> Result<bool> {
        Ok(cursor.peek()?.is_ascii_digit())
    }
}

impl Parse for LInt {
    #[inline]
    fn parse(cursor: &mut Cursor, _: &mut zinq_parse::ZinqParser) -> Result<Self> {
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

impl ToTokens for LInt {
    fn to_tokens(&self) -> zinq_error::Result<TokenStream> {
        Ok(Token::Literal(Literal::Int(self.clone())).into())
    }
}

#[cfg(test)]
mod test {
    use zinq_error::Result;
    use zinq_parse::{Parser, Span};

    use crate::zinq_parse::ZinqParser;

    #[test]
    fn is_int() -> Result<()> {
        let span = Span::from_bytes(b"103");
        let mut cursor = span.cursor();
        let mut parser = zinq_parse::ZinqParser;

        let mut token = parser.parse(&mut cursor)?;
        println!("{} => {}", token.name(), token.to_string());

        debug_assert!(token.is_int_literal());
        debug_assert_eq!(token.to_string(), "103");
        debug_assert_eq!(cursor.bytes(), b"");

        cursor = Span::from_bytes(b"103u16").cursor();
        token = parser.parse(&mut cursor)?;

        debug_assert!(token.is_int_literal());
        debug_assert_eq!(token.to_string(), "103u16");
        debug_assert_eq!(token.try_to_literal()?.try_to_int()?.digits(), b"103");
        debug_assert_eq!(token.try_to_literal()?.try_to_int()?.suffix(), b"u16");

        Ok(())
    }
}
