use zinq_parse::{Parse, Parser, Peek, Span};
use zinq_token::{Enclosed, LParen, Pub, RParen, Suffixed, Super, TokenParser};

use crate::{Node, Visibility};

///
/// ## Super Visibility
/// `pub super hello: string`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuperVisibility {
    pub span: Span,
    pub keyword: Suffixed<Pub, Enclosed<LParen, Super, RParen>>,
}

impl From<SuperVisibility> for Visibility {
    fn from(value: SuperVisibility) -> Self {
        Self::Super(value)
    }
}

impl Node for SuperVisibility {
    fn name(&self) -> &str {
        "Syntax::Visibility::Super"
    }

    fn accept<V: crate::Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for SuperVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for SuperVisibility {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse<TokenParser> for SuperVisibility {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        let keyword = parser.parse_as::<Suffixed<Pub, Enclosed<LParen, Super, RParen>>>(cursor)?;

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

    use crate::{SuperVisibility, TokenParser};

    #[test]
    fn should_parse() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(b"pub(super)").cursor();
        let value = parser.parse_as::<SuperVisibility>(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "pub(super)");
        debug_assert_eq!(value.keyword.suffix.to_string(), "(super)");

        Ok(())
    }
}
