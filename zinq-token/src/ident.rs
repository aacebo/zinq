use zinq_error::Result;
use zinq_parse::{Cursor, Parse, Parser, Peek, Span};

use crate::{Keyword, Token, TokenParser};

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
    fn peek(cursor: &Cursor, parser: &TokenParser) -> Result<bool> {
        Ok(cursor.peek()?.is_ascii_alphabetic())
    }
}

impl Parse<TokenParser> for Ident {
    #[inline]
    fn parse(cursor: &mut Cursor, parser: &mut TokenParser) -> Result<Token> {
        let span = cursor.next_while(|b, _| b.is_ascii_alphanumeric())?.span();

        if let Some(keyword) = Keyword::try_from_span(span) {
            return Ok(keyword.into());
        }

        Ok(Self { span: span.clone() }.into())
    }

    #[inline]
    fn span(&self) -> &Span {
        &self.span
    }
}

#[cfg(test)]
mod test {
    use zinq_error::Result;
    use zinq_parse::{Parser, Span};

    use crate::{Ident, TokenParser};

    #[test]
    fn should_parse() -> Result<()> {
        let span = Span::from_bytes(b"let test: string = \"test\"");
        let mut cursor = span.cursor();
        let mut parser = TokenParser;
        let mut token = parser.parse(&mut cursor)?;

        debug_assert!(token.is_keyword());
        debug_assert_eq!(token.to_string(), "let");

        token = parser.parse(&mut cursor)?;

        debug_assert!(token.is_ident());
        debug_assert_eq!(token.to_string(), "test");

        token = parser.parse(&mut cursor)?;

        debug_assert!(token.is_colon());
        debug_assert_eq!(token.to_string(), ":");

        token = parser.parse(&mut cursor)?;

        debug_assert!(token.is_ident());
        debug_assert_eq!(token.to_string(), "string");

        token = parser.parse(&mut cursor)?;

        debug_assert!(token.is_eq());
        debug_assert_eq!(token.to_string(), "=");

        token = parser.parse(&mut cursor)?;

        debug_assert!(token.is_literal_string());
        debug_assert_eq!(token.to_string(), "\"test\"");

        Ok(())
    }
}
