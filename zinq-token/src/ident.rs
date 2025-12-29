use zinq_error::Result;
use zinq_parse::{Cursor, Parse, Peek, Span};

use crate::{ToTokens, Token, TokenParser, TokenStream};

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

impl Peek<TokenParser> for Ident {
    #[inline]
    fn peek(cursor: &Cursor, _: &TokenParser) -> Result<bool> {
        Ok(cursor.peek()?.is_ascii_alphabetic())
    }
}

impl Parse<TokenParser> for Ident {
    #[inline]
    fn parse(cursor: &mut Cursor, _: &mut TokenParser) -> Result<Token> {
        let span = cursor.next_while(|b, _| b.is_ascii_alphanumeric())?.span();
        Ok(Self { span: span.clone() }.into())
    }

    #[inline]
    fn span(&self) -> &Span {
        &self.span
    }
}

impl ToTokens for Ident {
    fn to_tokens(&self) -> Result<TokenStream> {
        Ok(Token::Ident(self.clone()).into())
    }
}

#[cfg(test)]
mod test {
    use zinq_error::Result;
    use zinq_parse::{Parser, Span};

    use crate::TokenParser;

    #[test]
    fn should_parse() -> Result<()> {
        let span = Span::from_bytes(b"test");
        let mut cursor = span.cursor();
        let mut parser = TokenParser;
        let token = parser.parse(&mut cursor)?;

        debug_assert!(token.is_ident());
        debug_assert_eq!(token.to_string(), "test");

        Ok(())
    }
}
