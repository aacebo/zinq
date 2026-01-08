pub mod binary;
pub mod infix;
pub mod parser;
pub mod postfix;
pub mod prefix;
pub mod primary;
pub mod unary;

pub use binary::*;
pub use infix::*;
pub use parser::*;
pub use postfix::*;
pub use prefix::*;
pub use primary::*;
pub use unary::*;

use zinq_error::Result;
use zinq_parse::{Parse, Peek, Spanned};

use crate::{Node, Visitor};

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
    Not(NotExpr),
    Neg(NegExpr),

    /// ## Prefix
    Ref(RefExpr),

    /// ## Infix
    If(IfExpr),
    Match(MatchExpr),
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

    pub fn is_ref(&self) -> bool {
        match self {
            Self::Ref(_) => true,
            _ => false,
        }
    }

    pub fn is_not(&self) -> bool {
        match self {
            Self::Not(_) => true,
            _ => false,
        }
    }

    pub fn is_neg(&self) -> bool {
        match self {
            Self::Neg(_) => true,
            _ => false,
        }
    }

    pub fn is_if(&self) -> bool {
        match self {
            Self::If(_) => true,
            _ => false,
        }
    }

    pub fn is_match(&self) -> bool {
        match self {
            Self::Match(_) => true,
            _ => false,
        }
    }

    pub fn as_literal(&self) -> &LiteralExpr {
        match self {
            Self::Literal(v) => v,
            v => panic!("expected LiteralExpr, received {}", v.name()),
        }
    }

    pub fn as_ident(&self) -> &IdentExpr {
        match self {
            Self::Ident(v) => v,
            v => panic!("expected IdentExpr, received {}", v.name()),
        }
    }

    pub fn as_group(&self) -> &GroupExpr {
        match self {
            Self::Group(v) => v,
            v => panic!("expected GroupExpr, received {}", v.name()),
        }
    }

    pub fn as_arithmetic(&self) -> &ArithmeticExpr {
        match self {
            Self::Arithmetic(v) => v,
            v => panic!("expected ArithmeticExpr, received {}", v.name()),
        }
    }

    pub fn as_assign(&self) -> &AssignExpr {
        match self {
            Self::Assign(v) => v,
            v => panic!("expected AssignExpr, received {}", v.name()),
        }
    }

    pub fn as_cmp(&self) -> &CmpExpr {
        match self {
            Self::Cmp(v) => v,
            v => panic!("expected CmpExpr, received {}", v.name()),
        }
    }

    pub fn as_logical(&self) -> &LogicalExpr {
        match self {
            Self::Logical(v) => v,
            v => panic!("expected LogicalExpr, received {}", v.name()),
        }
    }

    pub fn as_call(&self) -> &CallExpr {
        match self {
            Self::Call(v) => v,
            v => panic!("expected CallExpr, received {}", v.name()),
        }
    }

    pub fn as_member(&self) -> &MemberExpr {
        match self {
            Self::Member(v) => v,
            v => panic!("expected MemberExpr, received {}", v.name()),
        }
    }

    pub fn as_ref(&self) -> &RefExpr {
        match self {
            Self::Ref(v) => v,
            v => panic!("expected RefExpr, received {}", v.name()),
        }
    }

    pub fn as_not(&self) -> &NotExpr {
        match self {
            Self::Not(v) => v,
            v => panic!("expected NotExpr, received {}", v.name()),
        }
    }

    pub fn as_neg(&self) -> &NegExpr {
        match self {
            Self::Neg(v) => v,
            v => panic!("expected NegExpr, received {}", v.name()),
        }
    }

    pub fn as_if(&self) -> &IfExpr {
        match self {
            Self::If(v) => v,
            v => panic!("expected IfExpr, received {}", v.name()),
        }
    }

    pub fn as_match(&self) -> &MatchExpr {
        match self {
            Self::Match(v) => v,
            v => panic!("expected MatchExpr, received {}", v.name()),
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
            Self::Ref(v) => v.name(),
            Self::Not(v) => v.name(),
            Self::Neg(v) => v.name(),
            Self::If(v) => v.name(),
            Self::Match(v) => v.name(),
        }
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
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
            Self::Ref(v) => write!(f, "{}", v),
            Self::Not(v) => write!(f, "{}", v),
            Self::Neg(v) => write!(f, "{}", v),
            Self::If(v) => write!(f, "{}", v),
            Self::Match(v) => write!(f, "{}", v),
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
}

impl Spanned for Expr {
    fn span(&self) -> zinq_parse::Span {
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
            Self::Ref(v) => v.span(),
            Self::Not(v) => v.span(),
            Self::Neg(v) => v.span(),
            Self::If(v) => v.span(),
            Self::Match(v) => v.span(),
        }
    }
}
