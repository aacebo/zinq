use zinq_parse::{Parse, Parser, Peek, Span};

use crate::TokenParser;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Suffixed<T, S>
where
    T: std::fmt::Display + Parse<TokenParser>,
    S: std::fmt::Display + Parse<TokenParser>,
{
    pub span: Span,
    pub suffix: S,
    pub inner: T,
}

impl<T, S> std::ops::Deref for Suffixed<T, S>
where
    T: std::fmt::Display + Parse<TokenParser>,
    S: std::fmt::Display + Parse<TokenParser>,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T, S> std::fmt::Display for Suffixed<T, S>
where
    T: std::fmt::Display + Parse<TokenParser>,
    S: std::fmt::Display + Parse<TokenParser>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl<T, S> Peek<TokenParser> for Suffixed<T, S>
where
    T: std::fmt::Display + Parse<TokenParser>,
    S: std::fmt::Display + Parse<TokenParser>,
{
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        if !parser.peek_as::<T>(cursor)? {
            return Ok(false);
        }

        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<T>(&mut fork) {
            Err(_) => return Ok(false),
            Ok(_) => {}
        };

        match fork_parser.parse_as::<S>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl<T, S> Parse<TokenParser> for Suffixed<T, S>
where
    T: std::fmt::Display + Parse<TokenParser>,
    S: std::fmt::Display + Parse<TokenParser>,
{
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        let inner = parser.parse_as::<T>(cursor)?;
        let suffix = parser.parse_as::<S>(cursor)?;

        Ok(Self {
            span: Span::from_bounds(inner.span(), suffix.span()),
            suffix,
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

    use crate::{LInt, Plus, Suffixed, TokenParser};

    #[test]
    fn should_parse_suffixed() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(b"66+").cursor();
        let stream: Suffixed<LInt, Plus> = parser.parse_as::<Suffixed<LInt, Plus>>(&mut cursor)?;

        debug_assert_eq!(stream.to_string(), "66+");

        Ok(())
    }
}
