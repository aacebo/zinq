use zinq_parse::{Parse, Parser, Peek, Span};
use zinq_token::{Enclosed, LParen, Mod, Pub, RParen, Suffixed, zinq_parse::ZinqParser};

use crate::{Node, Visibility};

///
/// ## Mod Visibility
/// `pub mod test: string`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModVisibility {
    pub span: Span,
    pub keyword: Suffixed<Pub, Enclosed<LParen, Mod, RParen>>,
}

impl From<ModVisibility> for Visibility {
    fn from(value: ModVisibility) -> Self {
        Self::Mod(value)
    }
}

impl Node for ModVisibility {
    fn name(&self) -> &str {
        "Syntax::Visibility::Mod"
    }

    fn accept<V: crate::Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for ModVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek for ModVisibility {
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

impl Parse for ModVisibility {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let keyword = parser.parse::<Suffixed<Pub, Enclosed<LParen, Mod, RParen>>>(cursor)?;

        Ok(Self {
            span: keyword.span().clone(),
            keyword,
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

    use crate::{ModVisibility, zinq_parse::ZinqParser};

    #[test]
    fn should_parse() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"pub(mod)").cursor();
        let value = parser.parse::<ModVisibility>(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "pub(mod)");
        debug_assert_eq!(value.keyword.suffix.to_string(), "(mod)");

        Ok(())
    }
}
