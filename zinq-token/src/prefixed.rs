use zinq_parse::{Parse, Peek, Span, Spanned};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Prefixed<P, T>
where
    P: std::fmt::Display + Parse,
    T: std::fmt::Display + Parse,
{
    pub prefix: P,
    pub inner: T,
}

impl<P, T> std::ops::Deref for Prefixed<P, T>
where
    P: std::fmt::Display + Parse,
    T: std::fmt::Display + Parse,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<P, T> std::fmt::Display for Prefixed<P, T>
where
    P: std::fmt::Display + Parse,
    T: std::fmt::Display + Parse,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl<P, T> Peek for Prefixed<P, T>
where
    P: std::fmt::Display + Parse,
    T: std::fmt::Display + Parse,
{
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        if !parser.peek::<P>(cursor)? {
            return Ok(false);
        }

        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse::<P>(&mut fork) {
            Err(_) => return Ok(false),
            Ok(_) => {}
        };

        match fork_parser.parse::<T>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl<P, T> Parse for Prefixed<P, T>
where
    P: std::fmt::Display + Parse,
    T: std::fmt::Display + Parse,
{
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let prefix = parser.parse::<P>(cursor)?;
        let inner = parser.parse::<T>(cursor)?;

        Ok(Self { prefix, inner })
    }
}

impl<P, T> Spanned for Prefixed<P, T>
where
    P: std::fmt::Display + Parse,
    T: std::fmt::Display + Parse,
{
    fn span(&self) -> Span {
        Span::join(self.prefix.span(), self.inner.span())
    }
}

#[cfg(test)]
mod test {
    use zinq_error::Result;
    use zinq_parse::Span;

    use crate::{LInt, Plus, Prefixed};

    #[test]
    fn should_parse_prefixed() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"+66").cursor();
        let stream: Prefixed<Plus, LInt> = parser.parse::<Prefixed<Plus, LInt>>(&mut cursor)?;

        debug_assert_eq!(stream.to_string(), "+66");

        Ok(())
    }
}
