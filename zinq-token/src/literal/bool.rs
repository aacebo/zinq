use zinq_error::Result;
use zinq_parse::{Cursor, Parse, Peek, Span, Spanned};

use crate::{Literal, ToTokens, Token, TokenStream};

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

    pub fn is_true(&self) -> bool {
        self.span.bytes() == b"true"
    }

    pub fn is_false(&self) -> bool {
        self.span.bytes() == b"false"
    }

    pub fn to_bool(&self) -> bool {
        self.span.to_string().parse().expect("expected boolean")
    }
}

impl std::fmt::Display for LBool {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek for LBool {
    #[inline]
    fn peek(cursor: &Cursor, _: &zinq_parse::ZinqParser) -> Result<bool> {
        Ok(cursor.peek_n(4).unwrap_or(&[]) == b"true"
            || cursor.peek_n(5).unwrap_or(&[]) == b"false")
    }
}

impl Parse for LBool {
    #[inline]
    fn parse(cursor: &mut Cursor, _: &mut zinq_parse::ZinqParser) -> Result<Self> {
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
}

impl Spanned for LBool {
    fn span(&self) -> Span {
        self.span.clone()
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
mod tests {
    use zinq_error::Result;
    use zinq_parse::Span;

    use crate::Token;

    #[test]
    fn is_bool() -> Result<()> {
        let span = Span::from_bytes(b"false");
        let mut cursor = span.cursor();
        let mut parser = zinq_parse::ZinqParser;
        let token = parser.parse::<Token>(&mut cursor)?;

        debug_assert!(token.is_bool_literal());
        debug_assert_eq!(token.to_string(), "false");

        Ok(())
    }
}
