use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::Literal;

use crate::{Node, expr::Expr};

///
/// ## Literal Expression
/// `"test"`
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LiteralExpr {
    pub value: Literal,
}

impl From<LiteralExpr> for Expr {
    fn from(value: LiteralExpr) -> Self {
        Self::Literal(value)
    }
}

impl Node for LiteralExpr {
    fn name(&self) -> &str {
        "Expr::Primary::Literal"
    }

    fn accept<V: crate::Visitor>(&self, visitor: &mut V) {
        visitor.visit_literal_expr(self);
    }
}

impl std::fmt::Display for LiteralExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
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
        Ok(Self { value })
    }
}

impl Spanned for LiteralExpr {
    fn span(&self) -> Span {
        self.value.span()
    }
}
