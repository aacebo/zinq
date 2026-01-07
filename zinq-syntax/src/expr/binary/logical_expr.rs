use zinq_parse::{Parse, Span};
use zinq_token::Logical;

use crate::{Node, Visitor, expr::Expr};

///
/// ## Logical Expression
/// `&&` or `||`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicalExpr {
    pub span: Span,
    pub left: Box<Expr>,
    pub op: Logical,
    pub right: Box<Expr>,
}

impl LogicalExpr {
    /// `<left> <op> <right>`
    pub fn new(left: Expr, op: Logical, right: Expr) -> Self {
        Self {
            span: Span::from_bounds(left.span(), right.span()),
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }

    pub fn span(&self) -> &Span {
        &self.span
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

    pub fn as_logical(&self) -> &LogicalExpr {
        match self {
            Self::Logical(v) => v,
            other => panic!("expected logical expression, found {}", other.name()),
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
        "Syntax::Expr::Binary::Logical"
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for LogicalExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

#[cfg(test)]
mod test {
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
