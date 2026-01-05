use zinq_parse::{Parse, Parser, Peek, Span};

use crate::zinq_parse::ZinqParser;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Punctuated<T, P>
where
    T: std::fmt::Display + Parse,
    P: std::fmt::Display + Parse,
{
    span: Span,
    segments: Vec<(T, Option<P>)>,
}

impl<T, P> std::ops::Deref for Punctuated<T, P>
where
    T: std::fmt::Display + Parse,
    P: std::fmt::Display + Parse,
{
    type Target = [(T, Option<P>)];

    fn deref(&self) -> &Self::Target {
        &self.segments
    }
}

impl<T, P> Punctuated<T, P>
where
    T: std::fmt::Display + Parse,
    P: std::fmt::Display + Parse,
{
    pub fn len(&self) -> usize {
        self.segments.len()
    }

    pub fn push(&mut self, segment: (T, Option<P>)) -> &mut Self {
        self.segments.push(segment);
        self
    }
}

impl<T, P> std::fmt::Display for Punctuated<T, P>
where
    T: std::fmt::Display + Parse,
    P: std::fmt::Display + Parse,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl<T, P> Peek for Punctuated<T, P>
where
    T: std::fmt::Display + Parse,
    P: std::fmt::Display + Parse,
{
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        parser.peek::<T>(cursor)
    }
}

impl<T, P> Parse for Punctuated<T, P>
where
    T: std::fmt::Display + Parse,
    P: std::fmt::Display + Parse,
{
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let mut segments = vec![];
        let start = cursor.span().clone();

        while parser.peek::<T>(cursor).unwrap_or(false) {
            let value = parser.parse::<T>(cursor)?;
            let mut delim: Option<P> = None;

            if parser.peek::<P>(cursor).unwrap_or(false) {
                delim = Some(parser.parse::<P>(cursor)?);
            }

            segments.push((value, delim.clone()));

            if let None = &delim {
                break;
            }
        }

        let end = cursor.span().clone();

        Ok(Self {
            span: Span::from_bounds(&start, &end),
            segments,
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

    use crate::{ColonColon, Comma, LInt, Punctuated, Token, zinq_parse::ZinqParser};

    #[test]
    fn should_parse_int_list() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"1, 2, 3").cursor();
        let stream = parser.parse::<Punctuated<LInt, Comma>>(&mut cursor)?;

        debug_assert_eq!(stream.len(), 3);
        debug_assert_eq!(stream.to_string(), "1, 2, 3");

        Ok(())
    }

    #[test]
    fn should_parse_ident_list() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"hello::world::mod").cursor();
        let stream = parser.parse::<Punctuated<Token, ColonColon>>(&mut cursor)?;

        debug_assert_eq!(stream.len(), 3);
        debug_assert_eq!(stream.to_string(), "hello::world::mod");

        Ok(())
    }
}
