mod assign;
mod binary;
mod get;
mod get_field;
mod group;
mod invoke;
mod literal;
mod logical;
mod r#ref;
mod set_field;
mod unary;

pub use assign::*;
pub use binary::*;
pub use get::*;
pub use get_field::*;
pub use group::*;
pub use invoke::*;
pub use literal::*;
pub use logical::*;
pub use r#ref::*;
pub use set_field::*;
pub use unary::*;

use zinq_error::Result;
use zinq_parse::{Parse, Parser, Peek};
use zinq_token::TokenParser;

use crate::{Node, Syntax, Visitor};

///
/// ## Expression
/// Something that can be evaluated, usually to a value (even if that value is `()`).
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Assign(AssignExpr),
    Binary(BinaryExpr),
    Group(GroupExpr),
    Invoke(InvokeExpr),
    Literal(LiteralExpr),
    Logical(LogicalExpr),
    GetField(GetFieldExpr),
    Get(GetExpr),
    Ref(RefExpr),
    SetField(SetFieldExpr),
    Unary(UnaryExpr),
}

impl Expr {
    pub fn is_assign(&self) -> bool {
        match self {
            Self::Assign(_) => true,
            _ => false,
        }
    }

    pub fn is_binary(&self) -> bool {
        match self {
            Self::Binary(_) => true,
            _ => false,
        }
    }

    pub fn is_group(&self) -> bool {
        match self {
            Self::Group(_) => true,
            _ => false,
        }
    }

    pub fn is_invoke(&self) -> bool {
        match self {
            Self::Invoke(_) => true,
            _ => false,
        }
    }

    pub fn is_literal(&self) -> bool {
        match self {
            Self::Literal(_) => true,
            _ => false,
        }
    }

    pub fn is_logical(&self) -> bool {
        match self {
            Self::Logical(_) => true,
            _ => false,
        }
    }

    pub fn is_get_field(&self) -> bool {
        match self {
            Self::GetField(_) => true,
            _ => false,
        }
    }

    pub fn is_get(&self) -> bool {
        match self {
            Self::Get(_) => true,
            _ => false,
        }
    }

    pub fn is_ref(&self) -> bool {
        match self {
            Self::Ref(_) => true,
            _ => false,
        }
    }

    pub fn is_set_field(&self) -> bool {
        match self {
            Self::SetField(_) => true,
            _ => false,
        }
    }

    pub fn is_unary(&self) -> bool {
        match self {
            Self::Unary(_) => true,
            _ => false,
        }
    }
}

impl Node for Expr {
    fn name(&self) -> &str {
        match self {
            Self::Assign(v) => v.name(),
            Self::Binary(v) => v.name(),
            Self::Group(v) => v.name(),
            Self::Invoke(v) => v.name(),
            Self::Literal(v) => v.name(),
            Self::Logical(v) => v.name(),
            Self::GetField(v) => v.name(),
            Self::Get(v) => v.name(),
            Self::Ref(v) => v.name(),
            Self::SetField(v) => v.name(),
            Self::Unary(v) => v.name(),
        }
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl From<Expr> for Syntax {
    fn from(value: Expr) -> Self {
        Self::Expr(value)
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Assign(v) => write!(f, "{}", v),
            Self::Binary(v) => write!(f, "{}", v),
            Self::Group(v) => write!(f, "{}", v),
            Self::Invoke(v) => write!(f, "{}", v),
            Self::Literal(v) => write!(f, "{}", v),
            Self::Logical(v) => write!(f, "{}", v),
            Self::GetField(v) => write!(f, "{}", v),
            Self::Get(v) => write!(f, "{}", v),
            Self::Ref(v) => write!(f, "{}", v),
            Self::SetField(v) => write!(f, "{}", v),
            Self::Unary(v) => write!(f, "{}", v),
        }
    }
}

impl Peek<TokenParser> for Expr {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse<TokenParser> for Expr {
    fn parse(cursor: &mut zinq_parse::Cursor, parser: &mut TokenParser) -> Result<Self> {
        if parser.peek_as::<RefExpr>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<RefExpr>(cursor)?.into());
        }

        if parser.peek_as::<GroupExpr>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<GroupExpr>(cursor)?.into());
        }

        if parser.peek_as::<AssignExpr>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<AssignExpr>(cursor)?.into());
        }

        if parser.peek_as::<LiteralExpr>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<LiteralExpr>(cursor)?.into());
        }

        if parser.peek_as::<GetExpr>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<GetExpr>(cursor)?.into());
        }

        if parser.peek_as::<GetFieldExpr>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<GetFieldExpr>(cursor)?.into());
        }

        if parser.peek_as::<SetFieldExpr>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<SetFieldExpr>(cursor)?.into());
        }

        if parser.peek_as::<LogicalExpr>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<LogicalExpr>(cursor)?.into());
        }

        if parser.peek_as::<UnaryExpr>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<UnaryExpr>(cursor)?.into());
        }

        if parser.peek_as::<BinaryExpr>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<BinaryExpr>(cursor)?.into());
        }

        if parser.peek_as::<InvokeExpr>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<InvokeExpr>(cursor)?.into());
        }

        Err(cursor.error(
            zinq_error::NOT_FOUND,
            &format!("unknown tokens '{}'", cursor),
        ))
    }

    fn span(&self) -> &zinq_parse::Span {
        match self {
            Self::Assign(v) => v.span(),
            Self::Binary(v) => v.span(),
            Self::Group(v) => v.span(),
            Self::Invoke(v) => v.span(),
            Self::Literal(v) => v.span(),
            Self::Logical(v) => v.span(),
            Self::GetField(v) => v.span(),
            Self::Get(v) => v.span(),
            Self::Ref(v) => v.span(),
            Self::SetField(v) => v.span(),
            Self::Unary(v) => v.span(),
        }
    }
}
