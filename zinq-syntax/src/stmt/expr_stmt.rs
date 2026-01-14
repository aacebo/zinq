use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::SemiColon;

use crate::{Syntax, expr::Expr, stmt::Stmt};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprStmt {
    pub expr: Expr,
    pub semi: Option<SemiColon>,
}

impl From<ExprStmt> for Stmt {
    fn from(value: ExprStmt) -> Self {
        Self::Expr(value)
    }
}

impl Syntax for ExprStmt {
    fn name(&self) -> &str {
        "Stmt::Expr"
    }

    fn accept<V: crate::Visitor>(&self, visitor: &mut V) {
        visitor.visit_expr_stmt(self);
        self.expr.accept(visitor);
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
        let semi = parser.parse::<Option<SemiColon>>(cursor)?;

        Ok(Self { expr, semi })
    }
}

impl Spanned for ExprStmt {
    fn span(&self) -> Span {
        if let Some(semi) = &self.semi {
            return Span::join(self.expr.span(), semi.span());
        }

        self.expr.span()
    }
}
