mod call;

pub use call::*;

use zinq_parse::ZinqParser;
use zinq_parse::{Parse, Parser, Peek};

use crate::{Node, expr::Expr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PostfixExpr {
    Call(CallExpr),
}

impl PostfixExpr {
    pub fn is_call(&self) -> bool {
        match self {
            Self::Call(_) => true,
        }
    }
}

impl From<PostfixExpr> for Expr {
    fn from(value: PostfixExpr) -> Self {
        Self::Postfix(value)
    }
}

impl std::fmt::Display for PostfixExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Call(v) => write!(f, "{}", v),
        }
    }
}

impl Node for PostfixExpr {
    fn name(&self) -> &str {
        match self {
            Self::Call(v) => v.name(),
        }
    }

    fn accept<V: crate::Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl Peek for PostfixExpr {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse for PostfixExpr {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        if parser.peek::<CallExpr>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<CallExpr>(cursor)?.into());
        }

        Err(cursor.error(
            zinq_error::NOT_FOUND,
            &format!("unknown tokens '{}'", cursor),
        ))
    }

    fn span(&self) -> &zinq_parse::Span {
        match self {
            Self::Call(v) => v.span(),
        }
    }
}
