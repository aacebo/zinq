use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::{Return, SemiColon};

use crate::{Node, expr::Expr, stmt::Stmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReturnStmt {
    pub keyword: Return,
    pub right: Expr,
    pub semi: SemiColon,
}

impl From<ReturnStmt> for Stmt {
    fn from(value: ReturnStmt) -> Self {
        Self::Return(value)
    }
}

impl Node for ReturnStmt {
    fn name(&self) -> &str {
        "Syntax::Stmt::Return"
    }

    fn accept<V: crate::Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for ReturnStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Peek for ReturnStmt {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser.peek::<Return>(cursor).unwrap_or(false))
    }
}

impl Parse for ReturnStmt {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let keyword = parser.parse::<Return>(cursor)?;
        let right = parser.parse::<Expr>(cursor)?;
        let semi = parser.parse::<SemiColon>(cursor)?;

        Ok(Self {
            keyword,
            right,
            semi,
        })
    }
}

impl Spanned for ReturnStmt {
    fn span(&self) -> Span {
        Span::join(self.keyword.span(), self.semi.span())
    }
}

#[cfg(test)]
mod test {
    use zinq_error::Result;
    use zinq_parse::Span;

    use crate::stmt::StmtParser;

    #[test]
    fn should_parse() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"return (1 + 2);").cursor();
        let stmt = parser.parse_stmt(&mut cursor)?;

        debug_assert!(stmt.is_return());
        debug_assert_eq!(stmt.to_string(), "return (1 + 2);");
        Ok(())
    }
}
