use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::If;

use crate::{Node, expr::Expr, stmt::Stmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfStmt {
    pub keyword: If,
    pub cond: Expr,
    pub then_stmt: Box<Stmt>,
    pub else_stmt: Option<Box<Stmt>>,
}

impl From<IfStmt> for Stmt {
    fn from(value: IfStmt) -> Self {
        Self::If(value)
    }
}

impl Node for IfStmt {
    fn name(&self) -> &str {
        "Syntax::Stmt::If"
    }

    fn accept<V: crate::Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for IfStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Peek for IfStmt {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser.peek::<If>(cursor).unwrap_or(false))
    }
}

impl Parse for IfStmt {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let keyword = parser.parse::<If>(cursor)?;
        let cond = parser.parse::<Expr>(cursor)?;
        let then_stmt = parser.parse::<Box<Stmt>>(cursor)?;
        let else_stmt = parser.parse::<Option<Box<Stmt>>>(cursor)?;

        Ok(Self {
            keyword,
            cond,
            then_stmt,
            else_stmt,
        })
    }
}

impl Spanned for IfStmt {
    fn span(&self) -> Span {
        if let Some(else_stmt) = &self.else_stmt {
            return Span::join(self.keyword.span(), else_stmt.span());
        }

        Span::join(self.keyword.span(), self.then_stmt.span())
    }
}

#[cfg(test)]
mod test {
    use zinq_error::Result;
    use zinq_parse::Span;

    use crate::stmt::StmtParser;

    #[test]
    fn should_parse_block() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"if 1 < 5 { return 1; }").cursor();
        let stmt = parser.parse_stmt(&mut cursor)?;

        // debug_assert!(stmt.is_if());
        debug_assert_eq!(stmt.to_string(), "if 1 < 5 { return 1; }");
        Ok(())
    }
}
