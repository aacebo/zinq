use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::{Else, If};

use crate::{
    Node,
    expr::Expr,
    stmt::{BlockStmt, Stmt, StmtParser},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IfStmt {
    pub keyword: If,
    pub cond: Expr,
    pub then_stmt: BlockStmt,
    pub else_stmt: Option<(Else, Box<Stmt>)>,
}

impl From<IfStmt> for Stmt {
    fn from(value: IfStmt) -> Self {
        Self::If(value)
    }
}

impl Node for IfStmt {
    fn name(&self) -> &str {
        "Stmt::If"
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
        let then_stmt = parser.parse::<BlockStmt>(cursor)?;

        if parser.peek::<Else>(cursor).unwrap_or(false) {
            let else_token = parser.parse::<Else>(cursor)?;
            let else_stmt = parser.parse_stmt(cursor)?;

            return Ok(Self {
                keyword,
                cond,
                then_stmt,
                else_stmt: Some((else_token, Box::new(else_stmt))),
            });
        }

        Ok(Self {
            keyword,
            cond,
            then_stmt,
            else_stmt: None,
        })
    }
}

impl Spanned for IfStmt {
    fn span(&self) -> Span {
        if let Some((_, stmt)) = &self.else_stmt {
            return Span::join(self.keyword.span(), stmt.span());
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

        debug_assert!(stmt.is_if());
        debug_assert_eq!(stmt.to_string(), "if 1 < 5 { return 1; }");
        Ok(())
    }

    #[test]
    fn should_parse_block_with_else() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"if 1 < 5 { return 1; } else { return -1; }").cursor();
        let stmt = parser.parse_stmt(&mut cursor)?;

        debug_assert!(stmt.is_if());
        debug_assert!(stmt.as_if().else_stmt.is_some());
        debug_assert_eq!(
            stmt.to_string(),
            "if 1 < 5 { return 1; } else { return -1; }"
        );

        Ok(())
    }

    #[test]
    fn should_parse_block_with_else_if() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(
            b"if 1 < 5 { return 1; } else if (i < 10) { return 0; } else { return -1; }",
        )
        .cursor();
        let stmt = parser.parse_stmt(&mut cursor)?;

        debug_assert!(stmt.is_if());
        debug_assert!(stmt.as_if().else_stmt.is_some());
        debug_assert!(stmt.as_if().else_stmt.as_ref().unwrap().1.is_if());
        debug_assert_eq!(
            stmt.to_string(),
            "if 1 < 5 { return 1; } else if (i < 10) { return 0; } else { return -1; }"
        );

        Ok(())
    }

    #[test]
    fn should_parse_is() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(
            b"if a is string {
                println(a);
            }",
        )
        .cursor();
        let stmt = parser.parse_stmt(&mut cursor)?;

        debug_assert!(stmt.is_if());
        debug_assert_eq!(
            stmt.to_string(),
            "if a is string {
                println(a);
            }",
        );

        Ok(())
    }
}
