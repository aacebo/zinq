use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::{LBrace, RBrace};

use crate::{Node, stmt::Stmt};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockStmt {
    pub left_brace: LBrace,
    pub stmts: Vec<Stmt>,
    pub right_brace: RBrace,
}

impl From<BlockStmt> for Stmt {
    fn from(value: BlockStmt) -> Self {
        Self::Block(value)
    }
}

impl Node for BlockStmt {
    fn name(&self) -> &str {
        "Stmt::Block"
    }
}

impl std::fmt::Display for BlockStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Peek for BlockStmt {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser.peek::<LBrace>(cursor).unwrap_or(false))
    }
}

impl Parse for BlockStmt {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let left_brace = parser.parse::<LBrace>(cursor)?;
        let mut stmts = vec![];

        while !cursor.eof() && !parser.peek::<RBrace>(cursor).unwrap_or(false) {
            stmts.push(parser.parse::<Stmt>(cursor)?);
        }

        let right_brace = parser.parse::<RBrace>(cursor)?;

        Ok(Self {
            left_brace,
            stmts,
            right_brace,
        })
    }
}

impl Spanned for BlockStmt {
    fn span(&self) -> Span {
        Span::join(self.left_brace.span(), self.right_brace.span())
    }
}
