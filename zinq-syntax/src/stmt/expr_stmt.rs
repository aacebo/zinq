use zinq_parse::{Parse, Peek, Span, Spanned};

use crate::{Node, expr::Expr, stmt::Stmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExprStmt {
    pub expr: Expr,
}

impl From<ExprStmt> for Stmt {
    fn from(value: ExprStmt) -> Self {
        Self::Expr(value)
    }
}

impl Node for ExprStmt {
    fn name(&self) -> &str {
        "Syntax::Stmt::Expr"
    }

    fn accept<V: crate::Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for ExprStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Peek for ExprStmt {
    fn peek(_: &zinq_parse::Cursor, _: &zinq_parse::ZinqParser) -> zinq_error::Result<bool> {
        Ok(true)
    }
}

impl Parse for ExprStmt {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let expr = parser.parse::<Expr>(cursor)?;
        Ok(Self { expr })
    }
}

impl Spanned for ExprStmt {
    fn span(&self) -> Span {
        self.expr.span()
    }
}
