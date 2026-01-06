pub mod binary;
pub mod parser;
pub mod postfix;
pub mod primary;
pub mod unary;

pub use binary::*;
pub use parser::*;
pub use postfix::*;
pub use primary::*;
pub use unary::*;

use zinq_error::Result;
use zinq_parse::{Parse, Peek};

use crate::{Node, Syntax, Visitor};

///
/// ## Expression
/// Something that can be evaluated, usually to a value (even if that value is `()`).
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    /// ## Primary
    Literal(LiteralExpr),
    Ident(IdentExpr),
    Group(GroupExpr),

    /// ## Binary
    Arithmetic(ArithmeticExpr),
    Assign(AssignExpr),
    Cmp(CmpExpr),
    Logical(LogicalExpr),

    /// ## Postfix
    Call(CallExpr),
    Member(MemberExpr),

    /// ## Unary
    Addr(AddrExpr),
    Not(NotExpr),
}

impl Expr {
    pub fn is_literal(&self) -> bool {
        match self {
            Self::Literal(_) => true,
            _ => false,
        }
    }

    pub fn is_ident(&self) -> bool {
        match self {
            Self::Ident(_) => true,
            _ => false,
        }
    }

    pub fn is_group(&self) -> bool {
        match self {
            Self::Group(_) => true,
            _ => false,
        }
    }

    pub fn is_arithmetic(&self) -> bool {
        match self {
            Self::Arithmetic(_) => true,
            _ => false,
        }
    }

    pub fn is_assign(&self) -> bool {
        match self {
            Self::Assign(_) => true,
            _ => false,
        }
    }

    pub fn is_cmp(&self) -> bool {
        match self {
            Self::Cmp(_) => true,
            _ => false,
        }
    }

    pub fn is_logical(&self) -> bool {
        match self {
            Self::Logical(_) => true,
            _ => false,
        }
    }

    pub fn is_call(&self) -> bool {
        match self {
            Self::Call(_) => true,
            _ => false,
        }
    }

    pub fn is_member(&self) -> bool {
        match self {
            Self::Member(_) => true,
            _ => false,
        }
    }

    pub fn is_addr(&self) -> bool {
        match self {
            Self::Addr(_) => true,
            _ => false,
        }
    }

    pub fn is_not(&self) -> bool {
        match self {
            Self::Not(_) => true,
            _ => false,
        }
    }
}

impl Node for Expr {
    fn name(&self) -> &str {
        match self {
            Self::Literal(v) => v.name(),
            Self::Ident(v) => v.name(),
            Self::Group(v) => v.name(),
            Self::Arithmetic(v) => v.name(),
            Self::Assign(v) => v.name(),
            Self::Cmp(v) => v.name(),
            Self::Logical(v) => v.name(),
            Self::Call(v) => v.name(),
            Self::Member(v) => v.name(),
            Self::Addr(v) => v.name(),
            Self::Not(v) => v.name(),
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
            Self::Literal(v) => write!(f, "{}", v),
            Self::Ident(v) => write!(f, "{}", v),
            Self::Group(v) => write!(f, "{}", v),
            Self::Arithmetic(v) => write!(f, "{}", v),
            Self::Assign(v) => write!(f, "{}", v),
            Self::Cmp(v) => write!(f, "{}", v),
            Self::Logical(v) => write!(f, "{}", v),
            Self::Call(v) => write!(f, "{}", v),
            Self::Member(v) => write!(f, "{}", v),
            Self::Addr(v) => write!(f, "{}", v),
            Self::Not(v) => write!(f, "{}", v),
        }
    }
}

impl Peek for Expr {
    fn peek(cursor: &zinq_parse::Cursor, parser: &zinq_parse::ZinqParser) -> Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse for Expr {
    fn parse(cursor: &mut zinq_parse::Cursor, parser: &mut zinq_parse::ZinqParser) -> Result<Self> {
        parser.parse_expr(cursor)
    }

    fn span(&self) -> &zinq_parse::Span {
        match self {
            Self::Literal(v) => v.span(),
            Self::Ident(v) => v.span(),
            Self::Group(v) => v.span(),
            Self::Arithmetic(v) => v.span(),
            Self::Assign(v) => v.span(),
            Self::Cmp(v) => v.span(),
            Self::Logical(v) => v.span(),
            Self::Call(v) => v.span(),
            Self::Member(v) => v.span(),
            Self::Addr(v) => v.span(),
            Self::Not(v) => v.span(),
        }
    }
}
