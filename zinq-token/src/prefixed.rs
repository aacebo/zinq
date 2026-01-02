use zinq_parse::{Parse, Parser, Peek, Span};

use crate::TokenParser;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Prefixed<P, T>
where
    P: std::fmt::Display + Parse<TokenParser>,
    T: std::fmt::Display + Parse<TokenParser>,
{
    pub span: Span,
    pub prefix: P,
    pub inner: T,
}

impl<P, T> std::ops::Deref for Prefixed<P, T>
where
    P: std::fmt::Display + Parse<TokenParser>,
    T: std::fmt::Display + Parse<TokenParser>,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<P, T> std::fmt::Display for Prefixed<P, T>
where
    P: std::fmt::Display + Parse<TokenParser>,
    T: std::fmt::Display + Parse<TokenParser>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl<P, T> Peek<TokenParser> for Prefixed<P, T>
where
    P: std::fmt::Display + Parse<TokenParser>,
    T: std::fmt::Display + Parse<TokenParser>,
{
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        if !parser.peek_as::<P>(cursor)? {
            return Ok(false);
        }

        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<P>(&mut fork) {
            Err(_) => return Ok(false),
            Ok(_) => {}
        };

        match fork_parser.parse_as::<T>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl<P, T> Parse<TokenParser> for Prefixed<P, T>
where
    P: std::fmt::Display + Parse<TokenParser>,
    T: std::fmt::Display + Parse<TokenParser>,
{
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        let prefix = parser.parse_as::<P>(cursor)?;
        let inner = parser.parse_as::<T>(cursor)?;

        Ok(Self {
            span: Span::from_bounds(prefix.span(), inner.span()),
            prefix,
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

    use crate::{LInt, Plus, Prefixed, TokenParser};

    #[test]
    fn should_parse_prefixed() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(b"+66").cursor();
        let stream: Prefixed<Plus, LInt> = parser.parse_as::<Prefixed<Plus, LInt>>(&mut cursor)?;

        debug_assert_eq!(stream.to_string(), "+66");

        Ok(())
    }
}
