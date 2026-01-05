pub mod expr;
pub mod fields;
pub mod param;
mod parser;
mod path;
pub mod stmt;
pub mod ty;
pub mod visibility;

pub use parser::*;
pub use path::*;
pub use visibility::*;

use zinq_error::Result;
use zinq_parse::{Parse, Peek};

use crate::{expr::Expr, stmt::Stmt, ty::Type};

pub trait Node: Parse {
    fn name(&self) -> &str;
    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> Result<()>
    where
        Self: Sized;
}

pub trait Visitor<N: Node> {
    fn visit(&mut self, node: &N) -> Result<()>;
}

///
/// ## Syntax
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Syntax {
    Expr(Expr),
    Stmt(Stmt),
    Type(Type),
    Visibility(Visibility),
}

impl std::fmt::Display for Syntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Expr(v) => write!(f, "{}", v),
            Self::Stmt(v) => write!(f, "{}", v),
            Self::Type(v) => write!(f, "{}", v),
            Self::Visibility(v) => write!(f, "{}", v),
        }
    }
}

impl Node for Syntax {
    fn name(&self) -> &str {
        match self {
            Self::Expr(v) => v.name(),
            Self::Stmt(v) => v.name(),
            Self::Type(v) => v.name(),
            Self::Visibility(v) => v.name(),
        }
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl Peek for Syntax {
    fn peek(cursor: &zinq_parse::Cursor, parser: &zinq_parse::ZinqParser) -> Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse for Syntax {
    fn parse(cursor: &mut zinq_parse::Cursor, parser: &mut zinq_parse::ZinqParser) -> Result<Self> {
        if parser.peek::<Visibility>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<Visibility>(cursor)?.into());
        }

        if parser.peek::<Type>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<Type>(cursor)?.into());
        }

        if parser.peek::<Expr>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<Expr>(cursor)?.into());
        }

        if parser.peek::<Stmt>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<Stmt>(cursor)?.into());
        }

        Err(cursor.error(
            zinq_error::NOT_FOUND,
            &format!("unknown tokens '{}'", cursor),
        ))
    }

    fn span(&self) -> &zinq_parse::Span {
        match self {
            Self::Expr(v) => v.span(),
            Self::Stmt(v) => v.span(),
            Self::Type(v) => v.span(),
            Self::Visibility(v) => v.span(),
        }
    }
}
