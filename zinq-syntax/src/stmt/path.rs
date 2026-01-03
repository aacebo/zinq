use zinq_parse::{Parse, Parser, Peek, Span};
use zinq_token::{ColonColon, Ident, Punctuated, TokenParser};

use crate::{Node, stmt::Stmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathStmt {
    pub span: Span,
    pub path: Punctuated<Ident, ColonColon>,
}

impl From<PathStmt> for Stmt {
    fn from(value: PathStmt) -> Self {
        Self::Path(value)
    }
}

impl Node for PathStmt {
    fn name(&self) -> &str {
        "Syntax::Stmt::Path"
    }

    fn accept<V: crate::Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for PathStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for PathStmt {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse<TokenParser> for PathStmt {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        let path = parser.parse_as::<Punctuated<Ident, ColonColon>>(cursor)?;

        Ok(Self {
            span: path.span().clone(),
            path,
        })
    }

    fn span(&self) -> &Span {
        &self.span
    }
}
