use zinq_parse::{Span, Spanned};
use zinq_token::Arithmetic;

use crate::{Node, expr::Expr};

///
/// ## Arithmetic Expression
/// `<left> <op> <right>`
/// ### Example
/// `<left> + <right>`
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArithmeticExpr {
    pub left: Box<Expr>,
    pub op: Arithmetic,
    pub right: Box<Expr>,
}

impl ArithmeticExpr {
    /// `<left> <op> <right>`
    pub fn new(left: Expr, op: Arithmetic, right: Expr) -> Self {
        Self {
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }
}

impl From<ArithmeticExpr> for Expr {
    fn from(value: ArithmeticExpr) -> Self {
        Self::Arithmetic(value)
    }
}

impl Node for ArithmeticExpr {
    fn name(&self) -> &str {
        "Expr::Binary::Arithmetic"
    }

    fn accept<V: crate::Visitor>(&self, visitor: &mut V) {
        visitor.visit_arithmetic_expr(self);
        self.left.accept(visitor);
        self.right.accept(visitor);
    }
}

impl std::fmt::Display for ArithmeticExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Spanned for ArithmeticExpr {
    fn span(&self) -> Span {
        Span::join(self.left.span(), self.right.span())
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
        let mut cursor = Span::from_bytes(b"a + 2").cursor();
        let value = parser.parse_expr(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "a + 2");
        debug_assert!(value.is_arithmetic());
        Ok(())
    }
}
