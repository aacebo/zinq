use zinq_parse::{Parse, Span};
use zinq_token::{Dot, Ident};

use crate::{Node, Visitor, expr::Expr};

///
/// ## Member Expression
/// `<target>.<name>`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemberExpr {
    pub span: Span,
    pub target: Box<Expr>,
    pub dot: Dot,
    pub name: Ident,
}

impl MemberExpr {
    /// `<target>.<name>`
    pub fn new(target: Expr, dot: Dot, name: Ident) -> Self {
        Self {
            span: Span::from_bounds(target.span(), name.span()),
            target: Box::new(target),
            dot,
            name,
        }
    }

    pub fn span(&self) -> &Span {
        &self.span
    }
}

impl From<MemberExpr> for Expr {
    fn from(value: MemberExpr) -> Self {
        Self::Member(value)
    }
}

impl Node for MemberExpr {
    fn name(&self) -> &str {
        "Syntax::Expr::Postfix::Member"
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for MemberExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}
