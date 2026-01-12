use zinq_parse::{Span, Spanned};
use zinq_token::{Dot, Ident};

use crate::{Node, expr::Expr};

///
/// ## Member Expression
/// `<target>.<name>`
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MemberExpr {
    pub target: Box<Expr>,
    pub dot: Dot,
    pub name: Ident,
}

impl MemberExpr {
    /// `<target>.<name>`
    pub fn new(target: Expr, dot: Dot, name: Ident) -> Self {
        Self {
            target: Box::new(target),
            dot,
            name,
        }
    }
}

impl From<MemberExpr> for Expr {
    fn from(value: MemberExpr) -> Self {
        Self::Member(value)
    }
}

impl Node for MemberExpr {
    fn name(&self) -> &str {
        "Expr::Postfix::Member"
    }
}

impl std::fmt::Display for MemberExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Spanned for MemberExpr {
    fn span(&self) -> Span {
        Span::join(self.target.span(), self.name.span())
    }
}

#[cfg(test)]
mod tests {
    use zinq_error::Result;
    use zinq_parse::Span;

    use crate::expr::ExprParser;

    #[test]
    fn should_parse() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"a.test().b").cursor();
        let value = parser.parse_expr(&mut cursor)?;

        debug_assert!(value.is_member());
        debug_assert_eq!(value.as_member().name.to_string(), "b");
        debug_assert_eq!(value.to_string(), "a.test().b");
        Ok(())
    }
}
