pub mod expr;
pub mod fields;
pub mod ty;

use zinq_error::Result;
use zinq_parse::{Parse, Parser, Peek};
use zinq_token::TokenParser;

use crate::expr::Expr;

pub trait Node: Parse<TokenParser> {
    fn name(&self) -> &str;
    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> Result<()>
    where
        Self: Sized;
}

pub trait Visitor<N: Node> {
    fn visit(&mut self, node: &N) -> Result<()>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Syntax {
    Expr(Expr),
}

impl std::fmt::Display for Syntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Expr(v) => write!(f, "{}", v),
        }
    }
}

impl Node for Syntax {
    fn name(&self) -> &str {
        match self {
            Self::Expr(v) => v.name(),
        }
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl Peek<TokenParser> for Syntax {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse<TokenParser> for Syntax {
    fn parse(cursor: &mut zinq_parse::Cursor, parser: &mut TokenParser) -> Result<Self> {
        if parser.peek_as::<Expr>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<Expr>(cursor)?.into());
        }

        Err(cursor.error(
            zinq_error::NOT_FOUND,
            &format!("unknown tokens '{}'", cursor),
        ))
    }

    fn span(&self) -> &zinq_parse::Span {
        match self {
            Self::Expr(v) => v.span(),
        }
    }
}
