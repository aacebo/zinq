mod block;
mod expr;
mod r#struct;

pub use block::*;
pub use expr::*;
pub use r#struct::*;

use zinq_error::Result;
use zinq_parse::{Parse, Parser, Peek};
use zinq_token::TokenParser;

use crate::{Node, Syntax, Visitor};

///
/// ## Statement
/// Something that can appear as a “line” inside a block
/// and doesn’t itself produce a value you can use in
/// a larger expression.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    Block(BlockStmt),
    Expr(ExprStmt),
    Struct(StructStmt),
}

impl From<Stmt> for Syntax {
    fn from(value: Stmt) -> Self {
        Self::Stmt(value)
    }
}

impl Node for Stmt {
    fn name(&self) -> &str {
        match self {
            Self::Block(v) => v.name(),
            Self::Expr(v) => v.name(),
            Self::Struct(v) => v.name(),
        }
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Block(v) => write!(f, "{}", v),
            Self::Expr(v) => write!(f, "{}", v),
            Self::Struct(v) => write!(f, "{}", v),
        }
    }
}

impl Peek<TokenParser> for Stmt {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse<TokenParser> for Stmt {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        if parser.peek_as::<StructStmt>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<StructStmt>(cursor)?.into());
        }

        if parser.peek_as::<BlockStmt>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<BlockStmt>(cursor)?.into());
        }

        if parser.peek_as::<ExprStmt>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<ExprStmt>(cursor)?.into());
        }

        Err(cursor.error(
            zinq_error::NOT_FOUND,
            &format!("unknown tokens '{}'", cursor),
        ))
    }

    fn span(&self) -> &zinq_parse::Span {
        match self {
            Self::Block(v) => v.span(),
            Self::Expr(v) => v.span(),
            Self::Struct(v) => v.span(),
        }
    }
}
