use zinq_parse::{Parse, Parser, Peek, Span};
use zinq_token::TokenParser;

use crate::{Node, Visibility};

///
/// ## Private Visibility
/// `test: string`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrivateVisibility {
    pub span: Span,
}

impl From<PrivateVisibility> for Visibility {
    fn from(value: PrivateVisibility) -> Self {
        Self::Priv(value)
    }
}

impl Node for PrivateVisibility {
    fn name(&self) -> &str {
        "Syntax::Visibility::Private"
    }

    fn accept<V: crate::Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for PrivateVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for PrivateVisibility {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse<TokenParser> for PrivateVisibility {
    fn parse(cursor: &mut zinq_parse::Cursor, _: &mut TokenParser) -> zinq_error::Result<Self> {
        Ok(Self {
            span: cursor.span().clone(),
        })
    }

    fn span(&self) -> &Span {
        &self.span
    }
}
