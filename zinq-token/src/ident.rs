use zinq_error::Result;
use zinq_parse::{Cursor, Parse, Peek, Span, Spanned};

use crate::{ToTokens, Token, TokenStream};

///
/// ## Ident
/// an identifier
/// ## Examples
/// - foo
/// - my_var
/// - MyType
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ident {
    span: Span,
}

impl Ident {
    pub fn name(&self) -> &'static str {
        "Ident"
    }
}

impl From<Ident> for Token {
    #[inline]
    fn from(value: Ident) -> Self {
        Self::Ident(value)
    }
}

impl std::fmt::Display for Ident {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek for Ident {
    #[inline]
    fn peek(cursor: &Cursor, _: &zinq_parse::ZinqParser) -> Result<bool> {
        Ok(cursor.peek()?.is_ascii_alphabetic() || cursor.peek()? == &b'_')
    }
}

impl Parse for Ident {
    #[inline]
    fn parse(cursor: &mut Cursor, _: &mut zinq_parse::ZinqParser) -> Result<Self> {
        let span = cursor
            .next_while(|b, _| b.is_ascii_alphanumeric() || b == &b'_')?
            .span();
        Ok(Self { span: span.clone() })
    }
}

impl Spanned for Ident {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl ToTokens for Ident {
    fn to_tokens(&self) -> Result<TokenStream> {
        Ok(Token::Ident(self.clone()).into())
    }
}

#[cfg(test)]
mod tests {
    use zinq_error::Result;
    use zinq_parse::Span;

    use crate::Token;

    #[test]
    fn should_parse() -> Result<()> {
        let span = Span::from_bytes(b"test");
        let mut cursor = span.cursor();
        let mut parser = zinq_parse::ZinqParser;
        let token = parser.parse::<Token>(&mut cursor)?;

        debug_assert!(token.is_ident());
        debug_assert_eq!(token.to_string(), "test");

        Ok(())
    }
}
