use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::{SemiColon, Use};

use crate::{Node, UsePath, Visibility, stmt::Stmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseStmt {
    pub vis: Visibility,
    pub keyword: Use,
    pub path: UsePath,
    pub semi: SemiColon,
}

impl From<UseStmt> for Stmt {
    fn from(value: UseStmt) -> Self {
        Self::Use(value)
    }
}

impl Node for UseStmt {
    fn name(&self) -> &str {
        "Syntax::Stmt::Use"
    }

    fn accept<V: crate::Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for UseStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Peek for UseStmt {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        fork_parser.parse::<Visibility>(&mut fork)?;
        Ok(fork_parser.peek::<Use>(&fork).unwrap_or(false))
    }
}

impl Parse for UseStmt {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let vis = parser.parse::<Visibility>(cursor)?;
        let keyword = parser.parse::<Use>(cursor)?;
        let path = parser.parse::<UsePath>(cursor)?;
        let semi = parser.parse::<SemiColon>(cursor)?;

        Ok(Self {
            vis,
            keyword,
            path,
            semi,
        })
    }
}

impl Spanned for UseStmt {
    fn span(&self) -> Span {
        Span::join(self.vis.span(), self.semi.span())
    }
}

#[cfg(test)]
mod test {
    use zinq_error::Result;
    use zinq_parse::Span;

    use crate::stmt::StmtParser;

    #[test]
    fn should_parse_ident() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"use std::string::String;").cursor();
        let stmt = parser.parse_stmt(&mut cursor)?;

        debug_assert_eq!(stmt.to_string(), "use std::string::String;");
        Ok(())
    }
}
