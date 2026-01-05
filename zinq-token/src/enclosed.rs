use zinq_parse::{Parse, Parser, Peek, Span};

use crate::zinq_parse::ZinqParser;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Enclosed<S, T, E>
where
    S: std::fmt::Display + Parse,
    T: std::fmt::Display + Parse,
    E: std::fmt::Display + Parse,
{
    pub span: Span,
    pub start: S,
    pub end: E,
    pub inner: T,
}

impl<S, T, E> std::ops::Deref for Enclosed<S, T, E>
where
    S: std::fmt::Display + Parse,
    T: std::fmt::Display + Parse,
    E: std::fmt::Display + Parse,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<S, T, E> std::fmt::Display for Enclosed<S, T, E>
where
    S: std::fmt::Display + Parse,
    T: std::fmt::Display + Parse,
    E: std::fmt::Display + Parse,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl<S, T, E> Peek for Enclosed<S, T, E>
where
    S: std::fmt::Display + Parse,
    T: std::fmt::Display + Parse,
    E: std::fmt::Display + Parse,
{
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        if !parser.peek::<S>(cursor)? {
            return Ok(false);
        }

        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse::<S>(&mut fork) {
            Err(_) => return Ok(false),
            Ok(_) => {}
        };

        match fork_parser.parse::<T>(&mut fork) {
            Err(_) => return Ok(false),
            Ok(_) => {}
        };

        match fork_parser.parse::<E>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl<S, T, E> Parse for Enclosed<S, T, E>
where
    S: std::fmt::Display + Parse,
    T: std::fmt::Display + Parse,
    E: std::fmt::Display + Parse,
{
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let start = parser.parse::<S>(cursor)?;
        let inner = parser.parse::<T>(cursor)?;
        let end = parser.parse::<E>(cursor)?;

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

    use crate::{Comma, Enclosed, LInt, LParen, Punctuated, RParen, zinq_parse::ZinqParser};

    #[test]
    fn should_parse_enclosed_int_list() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"(1, 2, 3)").cursor();
        let stream =
            parser.parse::<Enclosed<LParen, Punctuated<LInt, Comma>, RParen>>(&mut cursor)?;

        debug_assert_eq!(stream.len(), 3);
        debug_assert_eq!(stream.to_string(), "(1, 2, 3)");

        Ok(())
    }
}
