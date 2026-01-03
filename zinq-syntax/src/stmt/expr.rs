use zinq_parse::{Parse, Parser, Peek, Span};
use zinq_token::TokenParser;

use crate::{Node, expr::Expr, stmt::Stmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExprStmt {
    pub span: Span,
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
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for ExprStmt {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse<TokenParser> for ExprStmt {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        let expr = parser.parse_as::<Expr>(cursor)?;

        Ok(Self {
            span: expr.span().clone(),
            expr,
        })
    }

    fn span(&self) -> &Span {
        &self.span
    }
}
