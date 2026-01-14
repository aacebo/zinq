use zinq_parse::{Span, Spanned};
use zinq_token::Not;

use crate::{Syntax, expr::Expr};

///
/// ## Not Expression
/// `!(a == true)`
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NotExpr {
    pub not: Not,
    pub right: Box<Expr>,
}

impl NotExpr {
    /// `!<right>`
    pub fn new(not: Not, right: Expr) -> Self {
        Self {
            not,
            right: Box::new(right),
        }
    }
}

impl From<NotExpr> for Expr {
    fn from(value: NotExpr) -> Self {
        Self::Not(value)
    }
}

impl Syntax for NotExpr {
    fn name(&self) -> &str {
        "Expr::Unary::Not"
    }

    fn accept<V: crate::Visitor>(&self, visitor: &mut V) {
        visitor.visit_not_expr(self);
        self.right.accept(visitor);
    }
}

impl std::fmt::Display for NotExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Spanned for NotExpr {
    fn span(&self) -> Span {
        Span::join(self.not.span(), self.right.span())
    }
}

#[cfg(test)]
mod tests {
    use zinq_error::Result;
    use zinq_parse::Span;

    use crate::expr::ExprParser;

    #[test]
    fn should_parse_not() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"!b").cursor();
        let value = parser.parse_expr(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "!b");
        Ok(())
    }

    #[test]
    fn should_parse_not_of_group() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"!(a)").cursor();
        let value = parser.parse_expr(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "!(a)");
        Ok(())
    }
}
