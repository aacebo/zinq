use zinq_parse::{Parse, Parser, Peek, Span};
use zinq_token::{Literal, zinq_parse::ZinqParser};

use crate::{Node, Visitor, expr::PrimaryExpr};

///
/// ## Literal Expression
/// `"test"`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiteralExpr {
    pub span: Span,
    pub value: Literal,
}

impl From<LiteralExpr> for PrimaryExpr {
    fn from(value: LiteralExpr) -> Self {
        Self::Literal(value)
    }
}

impl Node for LiteralExpr {
    fn name(&self) -> &str {
        "Syntax::Expr::Primary::Literal"
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for LiteralExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek for LiteralExpr {
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

impl Parse for LiteralExpr {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let value = parser.parse::<Literal>(cursor)?;

        Ok(Self {
            span: value.span().clone(),
            value,
        })
    }

    fn span(&self) -> &Span {
        &self.span
    }
}
