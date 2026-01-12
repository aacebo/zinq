use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::Pub;

use crate::{Node, Visibility};

///
/// ## Public Visibility
/// `pub hello: string`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicVisibility {
    pub keyword: Pub,
}

impl From<PublicVisibility> for Visibility {
    fn from(value: PublicVisibility) -> Self {
        Self::Pub(value)
    }
}

impl Node for PublicVisibility {
    fn name(&self) -> &str {
        "Visibility::Public"
    }

    fn accept<V: crate::Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for PublicVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Peek for PublicVisibility {
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

impl Parse for PublicVisibility {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let keyword = parser.parse::<Pub>(cursor)?;

        Ok(Self { keyword })
    }
}

impl Spanned for PublicVisibility {
    fn span(&self) -> Span {
        self.keyword.span()
    }
}

#[cfg(test)]
mod test {
    use zinq_error::Result;
    use zinq_parse::Span;

    use crate::PublicVisibility;

    #[test]
    fn should_parse() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"pub").cursor();
        let value = parser.parse::<PublicVisibility>(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "pub");

        Ok(())
    }
}
