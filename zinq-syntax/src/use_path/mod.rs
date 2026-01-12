mod use_glob;
mod use_group;
mod use_name;
mod use_section;

pub use use_glob::*;
pub use use_group::*;
pub use use_name::*;
pub use use_section::*;

use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::{ColonColon, Ident};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UsePath {
    pub ident: Ident,
    pub delim: ColonColon,
    pub next: Box<UseSection>,
}

impl std::fmt::Display for UsePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Peek for UsePath {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        if !fork_parser.peek::<Ident>(&fork).unwrap_or(false) {
            return Ok(false);
        }

        fork_parser.parse::<Ident>(&mut fork)?;
        Ok(fork_parser.peek::<ColonColon>(&fork).unwrap_or(false))
    }
}

impl Parse for UsePath {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let ident = parser.parse::<Ident>(cursor)?;
        let delim = parser.parse::<ColonColon>(cursor)?;
        let next = parser.parse::<Box<UseSection>>(cursor)?;

        Ok(Self { ident, delim, next })
    }
}

impl Spanned for UsePath {
    fn span(&self) -> zinq_parse::Span {
        Span::join(self.ident.span(), self.next.span())
    }
}

#[cfg(test)]
mod tests {
    use zinq_error::Result;
    use zinq_parse::Span;

    use super::UsePath;

    #[test]
    fn should_parse_ident() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"std::string::String").cursor();
        let path = parser.parse::<UsePath>(&mut cursor)?;

        debug_assert_eq!(path.to_string(), "std::string::String");
        Ok(())
    }

    #[test]
    fn should_parse_glob() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"std::string::*").cursor();
        let path = parser.parse::<UsePath>(&mut cursor)?;

        debug_assert_eq!(path.to_string(), "std::string::*");
        Ok(())
    }

    #[test]
    fn should_parse_group() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"std::string(String, ToString)").cursor();
        let path = parser.parse::<UsePath>(&mut cursor)?;

        debug_assert_eq!(path.to_string(), "std::string(String, ToString)");
        Ok(())
    }

    #[test]
    fn should_parse_sub_group() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor =
            Span::from_bytes(b"std::string(parse::Parser, print::*, tokens(Token, ToTokens))")
                .cursor();
        let path = parser.parse::<UsePath>(&mut cursor)?;

        debug_assert_eq!(
            path.to_string(),
            "std::string(parse::Parser, print::*, tokens(Token, ToTokens))"
        );

        Ok(())
    }
}
