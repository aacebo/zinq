mod block;
mod expr;
mod r#fn;
mod r#impl;
mod r#let;
mod module;
mod r#struct;

pub use block::*;
pub use expr::*;
pub use r#fn::*;
pub use r#impl::*;
pub use r#let::*;
pub use module::*;
pub use r#struct::*;

use zinq_error::Result;
use zinq_parse::{Parse, Peek};

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
    Let(LetStmt),
    Mod(ModStmt),
    Struct(StructStmt),
    Fn(FnStmt),
    Impl(ImplStmt),
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
            Self::Let(v) => v.name(),
            Self::Mod(v) => v.name(),
            Self::Struct(v) => v.name(),
            Self::Fn(v) => v.name(),
            Self::Impl(v) => v.name(),
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
            Self::Let(v) => write!(f, "{}", v),
            Self::Mod(v) => write!(f, "{}", v),
            Self::Struct(v) => write!(f, "{}", v),
            Self::Fn(v) => write!(f, "{}", v),
            Self::Impl(v) => write!(f, "{}", v),
        }
    }
}

impl Peek for Stmt {
    fn peek(cursor: &zinq_parse::Cursor, parser: &zinq_parse::ZinqParser) -> Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse for Stmt {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        if parser.peek::<LetStmt>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<LetStmt>(cursor)?.into());
        }

        if parser.peek::<ImplStmt>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<ImplStmt>(cursor)?.into());
        }

        if parser.peek::<StructStmt>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<StructStmt>(cursor)?.into());
        }

        if parser.peek::<FnStmt>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<FnStmt>(cursor)?.into());
        }

        if parser.peek::<ModStmt>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<ModStmt>(cursor)?.into());
        }

        if parser.peek::<BlockStmt>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<BlockStmt>(cursor)?.into());
        }

        Ok(parser.parse::<ExprStmt>(cursor)?.into())
    }

    fn span(&self) -> &zinq_parse::Span {
        match self {
            Self::Block(v) => v.span(),
            Self::Expr(v) => v.span(),
            Self::Let(v) => v.span(),
            Self::Struct(v) => v.span(),
            Self::Mod(v) => v.span(),
            Self::Fn(v) => v.span(),
            Self::Impl(v) => v.span(),
        }
    }
}
