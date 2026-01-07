mod segment;

pub use segment::*;
use zinq_parse::{Parse, Peek};
use zinq_token::{ColonColon, Punctuated};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UsePath {
    pub segments: Punctuated<UseSegment, ColonColon>,
}

impl std::fmt::Display for UsePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.segments)
    }
}

impl Peek for UsePath {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse for UsePath {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let segments = parser.parse::<Punctuated<UseSegment, ColonColon>>(cursor)?;
        Ok(Self { segments })
    }

    fn span(&self) -> &zinq_parse::Span {
        self.segments.span()
    }
}

#[cfg(test)]
mod test {
    use zinq_error::Result;
    use zinq_parse::Span;

    use super::UsePath;

    #[test]
    fn should_parse_ident() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"std::string::String").cursor();
        let path = parser.parse::<UsePath>(&mut cursor)?;

        debug_assert_eq!(path.to_string(), "std::string::String");
        debug_assert_eq!(path.segments.len(), 3);
        debug_assert!(path.segments.last().unwrap().value().is_ident());

        Ok(())
    }

    #[test]
    fn should_parse_glob() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"std::string::*").cursor();
        let path = parser.parse::<UsePath>(&mut cursor)?;

        debug_assert_eq!(path.to_string(), "std::string::*");
        debug_assert_eq!(path.segments.len(), 3);
        debug_assert!(path.segments.last().unwrap().value().is_glob());

        Ok(())
    }

    #[test]
    fn should_parse_group() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"std::string::(String, ToString)").cursor();
        let path = parser.parse::<UsePath>(&mut cursor)?;

        debug_assert_eq!(path.to_string(), "std::string::(String, ToString)");
        debug_assert_eq!(path.segments.len(), 3);
        debug_assert!(path.segments.last().unwrap().value().is_group());

        Ok(())
    }
}
