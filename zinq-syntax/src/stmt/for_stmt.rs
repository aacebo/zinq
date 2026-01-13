use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::{For, In};

use crate::{
    Node,
    expr::Expr,
    pat::Pattern,
    stmt::{BlockStmt, Stmt},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForStmt {
    pub keyword: For,
    pub pattern: Pattern,
    pub in_keyword: In,
    pub expr: Expr,
    pub body: BlockStmt,
}

impl From<ForStmt> for Stmt {
    fn from(value: ForStmt) -> Self {
        Self::For(value)
    }
}

impl Node for ForStmt {
    fn name(&self) -> &str {
        "Stmt::For"
    }

    fn accept<V: crate::Visitor>(&self, visitor: &mut V) {
        visitor.visit_for_stmt(self);
        self.pattern.accept(visitor);
        self.expr.accept(visitor);
        self.body.accept(visitor);
    }
}

impl std::fmt::Display for ForStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Peek for ForStmt {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser.peek::<For>(cursor).unwrap_or(false))
    }
}

impl Parse for ForStmt {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let keyword = parser.parse::<For>(cursor)?;
        let pattern = parser.parse::<Pattern>(cursor)?;
        let in_keyword = parser.parse::<In>(cursor)?;
        let expr = parser.parse::<Expr>(cursor)?;
        let body = parser.parse::<BlockStmt>(cursor)?;

        Ok(Self {
            keyword,
            pattern,
            in_keyword,
            expr,
            body,
        })
    }
}

impl Spanned for ForStmt {
    fn span(&self) -> Span {
        Span::join(self.keyword.span(), self.body.span())
    }
}

#[cfg(test)]
mod tests {
    use zinq_error::Result;
    use zinq_parse::Span;

    use crate::stmt::StmtParser;

    #[test]
    fn should_parse_iter() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(
            b"for user in users {
                println(\"{user}\");
            }",
        )
        .cursor();

        let stmt = parser.parse_stmt(&mut cursor)?;

        debug_assert!(stmt.is_for());
        debug_assert_eq!(
            stmt.to_string(),
            "for user in users {
                println(\"{user}\");
            }"
        );

        Ok(())
    }

    #[test]
    fn should_parse_range() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(
            b"for i in [0..10] {
                println(\"{i}\");
            }",
        )
        .cursor();

        let stmt = parser.parse_stmt(&mut cursor)?;

        debug_assert!(stmt.is_for());
        debug_assert_eq!(
            stmt.to_string(),
            "for i in [0..10] {
                println(\"{i}\");
            }"
        );

        Ok(())
    }
}
