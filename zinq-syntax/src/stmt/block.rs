use zinq_parse::{Parse, Parser, Peek, Span};
use zinq_token::{LBrace, RBrace, TokenParser};

use crate::{Node, stmt::Stmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockStmt {
    pub span: Span,
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
        "Syntax::Stmt::Block"
    }

    fn accept<V: crate::Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for BlockStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for BlockStmt {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse<TokenParser> for BlockStmt {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        let left_brace = parser.parse_as::<LBrace>(cursor)?;
        let mut stmts = vec![];

        while !cursor.eof() {
            if parser.peek_as::<RBrace>(cursor).unwrap_or(true) {
                break;
            }

            stmts.push(parser.parse_as::<Stmt>(cursor)?);
        }

        let right_brace = parser.parse_as::<RBrace>(cursor)?;

        Ok(Self {
            span: Span::from_bounds(left_brace.span(), right_brace.span()),
            left_brace,
            stmts,
            right_brace,
        })
    }

    fn span(&self) -> &Span {
        &self.span
    }
}
