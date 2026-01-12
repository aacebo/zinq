use zinq_parse::{Parse, Peek, Span, Spanned};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Suffixed<T, S>
where
    T: std::fmt::Display + Parse,
    S: std::fmt::Display + Parse,
{
    pub suffix: S,
    pub inner: T,
}

impl<T, S> std::ops::Deref for Suffixed<T, S>
where
    T: std::fmt::Display + Parse,
    S: std::fmt::Display + Parse,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T, S> std::fmt::Display for Suffixed<T, S>
where
    T: std::fmt::Display + Parse,
    S: std::fmt::Display + Parse,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl<T, S> Peek for Suffixed<T, S>
where
    T: std::fmt::Display + Parse,
    S: std::fmt::Display + Parse,
{
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        if !parser.peek::<T>(cursor)? {
            return Ok(false);
        }

        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse::<T>(&mut fork) {
            Err(_) => return Ok(false),
            Ok(_) => {}
        };

        match fork_parser.parse::<S>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl<T, S> Parse for Suffixed<T, S>
where
    T: std::fmt::Display + Parse,
    S: std::fmt::Display + Parse,
{
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let inner = parser.parse::<T>(cursor)?;
        let suffix = parser.parse::<S>(cursor)?;

        Ok(Self { suffix, inner })
    }
}

impl<T, S> Spanned for Suffixed<T, S>
where
    T: std::fmt::Display + Parse,
    S: std::fmt::Display + Parse,
{
    fn span(&self) -> Span {
        Span::join(self.inner.span(), self.suffix.span())
    }
}

#[cfg(test)]
mod test {
    use zinq_error::Result;
    use zinq_parse::Span;

    use crate::{LInt, Plus, Suffixed};

    #[test]
    fn should_parse_suffixed() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"66+").cursor();
        let stream: Suffixed<LInt, Plus> = parser.parse::<Suffixed<LInt, Plus>>(&mut cursor)?;

        debug_assert_eq!(stream.to_string(), "66+");

        Ok(())
    }
}
