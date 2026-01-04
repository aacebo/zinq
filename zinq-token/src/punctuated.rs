use zinq_parse::{Parse, Parser, Peek, Span};

use crate::TokenParser;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Punctuated<T, P>
where
    T: std::fmt::Display + Parse<TokenParser>,
    P: std::fmt::Display + Parse<TokenParser>,
{
    span: Span,
    segments: Vec<(T, Option<P>)>,
}

impl<T, P> std::ops::Deref for Punctuated<T, P>
where
    T: std::fmt::Display + Parse<TokenParser>,
    P: std::fmt::Display + Parse<TokenParser>,
{
    type Target = [(T, Option<P>)];

    fn deref(&self) -> &Self::Target {
        &self.segments
    }
}

impl<T, P> Punctuated<T, P>
where
    T: std::fmt::Display + Parse<TokenParser>,
    P: std::fmt::Display + Parse<TokenParser>,
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
    T: std::fmt::Display + Parse<TokenParser>,
    P: std::fmt::Display + Parse<TokenParser>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl<T, P> Peek<TokenParser> for Punctuated<T, P>
where
    T: std::fmt::Display + Parse<TokenParser>,
    P: std::fmt::Display + Parse<TokenParser>,
{
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        parser.peek_as::<T>(cursor)
    }
}

impl<T, P> Parse<TokenParser> for Punctuated<T, P>
where
    T: std::fmt::Display + Parse<TokenParser>,
    P: std::fmt::Display + Parse<TokenParser>,
{
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        let mut segments = vec![];
        let start = cursor.span().clone();

        while parser.peek_as::<T>(cursor).unwrap_or(false) {
            let value = parser.parse_as::<T>(cursor)?;
            let mut delim: Option<P> = None;

            if parser.peek_as::<P>(cursor).unwrap_or(false) {
                delim = Some(parser.parse_as::<P>(cursor)?);
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

    use crate::{ColonColon, Comma, LInt, Punctuated, Token, TokenParser};

    #[test]
    fn should_parse_int_list() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(b"1, 2, 3").cursor();
        let stream = parser.parse_as::<Punctuated<LInt, Comma>>(&mut cursor)?;

        debug_assert_eq!(stream.len(), 3);
        debug_assert_eq!(stream.to_string(), "1, 2, 3");

        Ok(())
    }

    #[test]
    fn should_parse_ident_list() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(b"hello::world::mod").cursor();
        let stream = parser.parse_as::<Punctuated<Token, ColonColon>>(&mut cursor)?;

        debug_assert_eq!(stream.len(), 3);
        debug_assert_eq!(stream.to_string(), "hello::world::mod");

        Ok(())
    }
}
