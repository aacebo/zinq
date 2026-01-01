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
        for (i, (token, punct)) in self.segments.iter().enumerate() {
            write!(f, "{}", token)?;

            if i < self.segments.len() - 1
                && let Some(p) = punct
            {
                write!(f, "{}", p)?;
            }
        }

        Ok(())
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

// impl<T, P> Parse<TokenParser> for Punctuated<T, P>
// where
//     T: std::fmt::Display + Parse<TokenParser>,
//     P: std::fmt::Display + Parse<TokenParser>,
// {
//     fn parse(cursor: &mut zinq_parse::Cursor, parser: &mut TokenParser) -> zinq_error::Result<Token> {
//         let mut segments = vec![];

//         while !cursor.eof() {
//             let value = parser.parse_as::<T>(cursor)?;
//             let mut delim: Option<Token> = None;

//             if parser.peek_as::<P>(cursor)? {
//                 delim = Some(parser.parse_as::<P>(cursor)?);
//             }

//             segments.push((value));
//         }
//     }

//     fn span(&self) -> &Span {
//         &self.span
//     }
// }

#[cfg(test)]
mod test {
    use zinq_error::Result;

    #[test]
    fn should_parse_path() -> Result<()> {
        Ok(())
    }
}
