use zinq_parse::{Parse, Parser, Peek, Span};

use crate::TokenParser;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Enclosed<S, T, E>
where
    S: std::fmt::Display + Parse<TokenParser>,
    T: std::fmt::Display + Parse<TokenParser>,
    E: std::fmt::Display + Parse<TokenParser>,
{
    pub span: Span,
    pub start: S,
    pub end: E,
    pub inner: T,
}

impl<S, T, E> std::ops::Deref for Enclosed<S, T, E>
where
    S: std::fmt::Display + Parse<TokenParser>,
    T: std::fmt::Display + Parse<TokenParser>,
    E: std::fmt::Display + Parse<TokenParser>,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<S, T, E> std::fmt::Display for Enclosed<S, T, E>
where
    S: std::fmt::Display + Parse<TokenParser>,
    T: std::fmt::Display + Parse<TokenParser>,
    E: std::fmt::Display + Parse<TokenParser>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl<S, T, E> Peek<TokenParser> for Enclosed<S, T, E>
where
    S: std::fmt::Display + Parse<TokenParser>,
    T: std::fmt::Display + Parse<TokenParser>,
    E: std::fmt::Display + Parse<TokenParser>,
{
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        if !parser.peek_as::<S>(cursor)? {
            return Ok(false);
        }

        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<S>(&mut fork) {
            Err(_) => return Ok(false),
            Ok(_) => {}
        };

        match fork_parser.parse_as::<T>(&mut fork) {
            Err(_) => return Ok(false),
            Ok(_) => {}
        };

        match fork_parser.parse_as::<E>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl<S, T, E> Parse<TokenParser> for Enclosed<S, T, E>
where
    S: std::fmt::Display + Parse<TokenParser>,
    T: std::fmt::Display + Parse<TokenParser>,
    E: std::fmt::Display + Parse<TokenParser>,
{
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        let start = parser.parse_as::<S>(cursor)?;
        let inner = parser.parse_as::<T>(cursor)?;
        let end = parser.parse_as::<E>(cursor)?;

        Ok(Self {
            span: Span::from_bounds(start.span(), end.span()),
            start,
            end,
            inner,
        })
    }

    fn span(&self) -> &Span {
        &self.span
    }
}

#[cfg(test)]
mod test {
    use zinq_error::Result;
    use zinq_parse::{Parser, Span};

    use crate::{Comma, Enclosed, LInt, LParen, Punctuated, RParen, TokenParser};

    #[test]
    fn should_parse_enclosed_int_list() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(b"(1, 2, 3)").cursor();
        let stream =
            parser.parse_as::<Enclosed<LParen, Punctuated<LInt, Comma>, RParen>>(&mut cursor)?;

        debug_assert_eq!(stream.len(), 3);
        debug_assert_eq!(stream.to_string(), "(1, 2, 3)");

        Ok(())
    }
}
