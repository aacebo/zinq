use zinq_parse::{Span, Spanned};
use zinq_token::Logical;

use crate::{Node, expr::Expr};

///
/// ## Logical Expression
/// `&&` or `||`
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LogicalExpr {
    pub left: Box<Expr>,
    pub op: Logical,
    pub right: Box<Expr>,
}

impl LogicalExpr {
    /// `<left> <op> <right>`
    pub fn new(left: Expr, op: Logical, right: Expr) -> Self {
        Self {
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }
}

impl Expr {
    pub fn is_or(&self) -> bool {
        match self {
            Self::Logical(v) => v.op.is_or(),
            _ => false,
        }
    }

    pub fn is_and(&self) -> bool {
        match self {
            Self::Logical(v) => v.op.is_and(),
            _ => false,
        }
    }
}

impl From<LogicalExpr> for Expr {
    fn from(value: LogicalExpr) -> Self {
        Self::Logical(value)
    }
}

impl Node for LogicalExpr {
    fn name(&self) -> &str {
        "Expr::Binary::Logical"
    }
}

impl std::fmt::Display for LogicalExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Spanned for LogicalExpr {
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
        let mut cursor = Span::from_bytes(b"a || true").cursor();
        let value = parser.parse_expr(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "a || true");
        debug_assert!(value.is_logical());
        Ok(())
    }
}
