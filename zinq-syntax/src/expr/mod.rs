pub mod binary;
pub mod infix;
pub mod parser;
pub mod postfix;
pub mod prefix;
pub mod primary;
pub mod unary;
mod visitor;

pub use binary::*;
pub use infix::*;
pub use parser::*;
pub use postfix::*;
pub use prefix::*;
pub use primary::*;
pub use unary::*;
pub use visitor::*;

use zinq_error::Result;
use zinq_parse::{Parse, Peek, Spanned};

use crate::Syntax;

///
/// ## Expression
/// Something that can be evaluated, usually to a value (even if that value is `()`).
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    /// ## Primary
    Literal(LiteralExpr),
    Path(PathExpr),
    Group(GroupExpr),
    Struct(StructExpr),
    Tuple(TupleExpr),
    Array(ArrayExpr),
    Range(RangeExpr),

    /// ## Binary
    Arithmetic(ArithmeticExpr),
    Assign(AssignExpr),
    Cmp(CmpExpr),
    Is(IsExpr),
    Logical(LogicalExpr),

    /// ## Postfix
    Call(CallExpr),
    Index(IndexExpr),
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

    pub fn is_path(&self) -> bool {
        match self {
            Self::Path(_) => true,
            _ => false,
        }
    }

    pub fn is_group(&self) -> bool {
        match self {
            Self::Group(_) => true,
            _ => false,
        }
    }

    pub fn is_struct(&self) -> bool {
        match self {
            Self::Struct(_) => true,
            _ => false,
        }
    }

    pub fn is_tuple(&self) -> bool {
        match self {
            Self::Tuple(_) => true,
            _ => false,
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            Self::Array(_) => true,
            _ => false,
        }
    }

    pub fn is_range(&self) -> bool {
        match self {
            Self::Range(_) => true,
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

    pub fn is_is(&self) -> bool {
        match self {
            Self::Is(_) => true,
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

    pub fn is_index(&self) -> bool {
        match self {
            Self::Index(_) => true,
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

    pub fn as_path(&self) -> &PathExpr {
        match self {
            Self::Path(v) => v,
            v => panic!("expected PathExpr, received {}", v.name()),
        }
    }

    pub fn as_group(&self) -> &GroupExpr {
        match self {
            Self::Group(v) => v,
            v => panic!("expected GroupExpr, received {}", v.name()),
        }
    }

    pub fn as_struct(&self) -> &StructExpr {
        match self {
            Self::Struct(v) => v,
            v => panic!("expected StructExpr, received {}", v.name()),
        }
    }

    pub fn as_tuple(&self) -> &TupleExpr {
        match self {
            Self::Tuple(v) => v,
            v => panic!("expected TupleExpr, received {}", v.name()),
        }
    }

    pub fn as_array(&self) -> &ArrayExpr {
        match self {
            Self::Array(v) => v,
            v => panic!("expected ArrayExpr, received {}", v.name()),
        }
    }

    pub fn as_range(&self) -> &RangeExpr {
        match self {
            Self::Range(v) => v,
            v => panic!("expected RangeExpr, received {}", v.name()),
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

    pub fn as_is(&self) -> &IsExpr {
        match self {
            Self::Is(v) => v,
            v => panic!("expected IsExpr, received {}", v.name()),
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

    pub fn as_index(&self) -> &IndexExpr {
        match self {
            Self::Index(v) => v,
            v => panic!("expected IndexExpr, received {}", v.name()),
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

impl Syntax for Expr {
    fn name(&self) -> &str {
        match self {
            Self::Literal(v) => v.name(),
            Self::Path(v) => v.name(),
            Self::Group(v) => v.name(),
            Self::Struct(v) => v.name(),
            Self::Tuple(v) => v.name(),
            Self::Array(v) => v.name(),
            Self::Range(v) => v.name(),
            Self::Arithmetic(v) => v.name(),
            Self::Assign(v) => v.name(),
            Self::Cmp(v) => v.name(),
            Self::Is(v) => v.name(),
            Self::Logical(v) => v.name(),
            Self::Call(v) => v.name(),
            Self::Index(v) => v.name(),
            Self::Member(v) => v.name(),
            Self::Ref(v) => v.name(),
            Self::Not(v) => v.name(),
            Self::Neg(v) => v.name(),
            Self::If(v) => v.name(),
            Self::Match(v) => v.name(),
        }
    }

    fn accept<V: crate::Visitor>(&self, visitor: &mut V) {
        visitor.visit_expr(self);

        match self {
            Self::Arithmetic(v) => v.accept(visitor),
            Self::Array(v) => v.accept(visitor),
            Self::Assign(v) => v.accept(visitor),
            Self::Call(v) => v.accept(visitor),
            Self::Cmp(v) => v.accept(visitor),
            Self::Group(v) => v.accept(visitor),
            Self::If(v) => v.accept(visitor),
            Self::Index(v) => v.accept(visitor),
            Self::Is(v) => v.accept(visitor),
            Self::Literal(v) => v.accept(visitor),
            Self::Logical(v) => v.accept(visitor),
            Self::Match(v) => v.accept(visitor),
            Self::Member(v) => v.accept(visitor),
            Self::Neg(v) => v.accept(visitor),
            Self::Not(v) => v.accept(visitor),
            Self::Path(v) => v.accept(visitor),
            Self::Range(v) => v.accept(visitor),
            Self::Ref(v) => v.accept(visitor),
            Self::Struct(v) => v.accept(visitor),
            Self::Tuple(v) => v.accept(visitor),
        }
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(v) => write!(f, "{}", v),
            Self::Path(v) => write!(f, "{}", v),
            Self::Group(v) => write!(f, "{}", v),
            Self::Struct(v) => write!(f, "{}", v),
            Self::Tuple(v) => write!(f, "{}", v),
            Self::Array(v) => write!(f, "{}", v),
            Self::Range(v) => write!(f, "{}", v),
            Self::Arithmetic(v) => write!(f, "{}", v),
            Self::Assign(v) => write!(f, "{}", v),
            Self::Cmp(v) => write!(f, "{}", v),
            Self::Is(v) => write!(f, "{}", v),
            Self::Logical(v) => write!(f, "{}", v),
            Self::Call(v) => write!(f, "{}", v),
            Self::Index(v) => write!(f, "{}", v),
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
            Self::Path(v) => v.span(),
            Self::Group(v) => v.span(),
            Self::Struct(v) => v.span(),
            Self::Tuple(v) => v.span(),
            Self::Array(v) => v.span(),
            Self::Range(v) => v.span(),
            Self::Arithmetic(v) => v.span(),
            Self::Assign(v) => v.span(),
            Self::Cmp(v) => v.span(),
            Self::Is(v) => v.span(),
            Self::Logical(v) => v.span(),
            Self::Call(v) => v.span(),
            Self::Index(v) => v.span(),
            Self::Member(v) => v.span(),
            Self::Ref(v) => v.span(),
            Self::Not(v) => v.span(),
            Self::Neg(v) => v.span(),
            Self::If(v) => v.span(),
            Self::Match(v) => v.span(),
        }
    }
}
