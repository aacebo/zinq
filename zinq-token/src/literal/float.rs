use zinq_error::Result;
use zinq_parse::{Cursor, Parse, Peek, Span, Spanned};

use crate::{Literal, ToTokens, Token, TokenStream};

///
/// ## LFloat
/// a literal float
/// ### Example
/// `0.1` or `0.1f32` or `0.1f64` etc
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LFloat {
    span: Span,
}

impl LFloat {
    pub fn name(&self) -> &'static str {
        "LFloat"
    }

    pub fn is_f32(&self) -> bool {
        self.span.bytes().ends_with(b"f32")
    }

    pub fn is_f64(&self) -> bool {
        self.span.bytes().ends_with(b"f64")
    }

    pub fn digits(&self) -> &[u8] {
        if self.is_f32() || self.is_f64() {
            return &self.span.bytes()[0..self.span.len() - 3];
        }

        &self.span.bytes()
    }

    pub fn suffix(&self) -> &[u8] {
        if self.is_f32() || self.is_f64() {
            return &self.span.bytes()[self.span.len() - 3..self.span.len()];
        }

        &[]
    }
}

impl Literal {
    pub fn is_f32(&self) -> bool {
        match self {
            Self::Float(v) => v.is_f32(),
            _ => false,
        }
    }

    pub fn is_f64(&self) -> bool {
        match self {
            Self::Float(v) => v.is_f64(),
            _ => false,
        }
    }
}

impl Token {
    pub fn is_f32_literal(&self) -> bool {
        self.is_literal() && self.try_to_literal().expect("must be literal").is_f32()
    }

    pub fn is_f64_literal(&self) -> bool {
        self.is_literal() && self.try_to_literal().expect("must be literal").is_f64()
    }
}

impl std::fmt::Display for LFloat {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek for LFloat {
    #[inline]
    fn peek(cursor: &Cursor, _: &zinq_parse::ZinqParser) -> Result<bool> {
        if !cursor.peek()?.is_ascii_digit() {
            return Ok(false);
        }

        let mut fork = cursor.fork();
        fork.next_while(|b, _| b.is_ascii_digit())?;

        if fork.peek().unwrap_or(&0) != &b'.' {
            return Ok(false);
        }

        Ok(fork.next()?.peek().unwrap_or(&0).is_ascii_digit())
    }
}

impl Parse for LFloat {
    #[inline]
    fn parse(cursor: &mut Cursor, _: &mut zinq_parse::ZinqParser) -> Result<Self> {
        cursor
            .next_while(|b, _| b.is_ascii_digit())?
            .next_if(|b, _| b == &b'.')?
            .next_while(|b, _| b.is_ascii_digit())?;

        if cursor.peek_n(3).unwrap_or(&[]) == b"f32" || cursor.peek_n(3).unwrap_or(&[]) == b"f64" {
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
}

impl Spanned for LFloat {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl From<LFloat> for Literal {
    fn from(value: LFloat) -> Self {
        Self::Float(value)
    }
}

impl From<LFloat> for Token {
    fn from(value: LFloat) -> Self {
        Self::Literal(Literal::Float(value))
    }
}

impl ToTokens for LFloat {
    fn to_tokens(&self) -> zinq_error::Result<TokenStream> {
        Ok(Token::Literal(Literal::Float(self.clone())).into())
    }
}

#[cfg(test)]
mod test {
    use zinq_error::Result;
    use zinq_parse::Span;

    use crate::Token;

    #[test]
    fn is_float() -> Result<()> {
        let span = Span::from_bytes(b"103.63f64");
        let mut cursor = span.cursor();
        let mut parser = zinq_parse::ZinqParser;

        let token = parser.parse::<Token>(&mut cursor)?;
        println!("{} => {}", token.name(), token.to_string());

        debug_assert!(token.is_float_literal());
        debug_assert_eq!(token.to_string(), "103.63f64");
        debug_assert_eq!(token.try_to_literal()?.try_to_float()?.suffix(), b"f64");

        Ok(())
    }
}
