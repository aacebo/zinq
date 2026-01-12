use zinq_parse::{Span, Spanned};
use zinq_token::Cmp;

use crate::{Node, expr::Expr};

///
/// ## Cmp Expression
/// `<left> <op> <right>`
/// ### Example
/// `<left> >= <right>`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmpExpr {
    pub left: Box<Expr>,
    pub op: Cmp,
    pub right: Box<Expr>,
}

impl CmpExpr {
    /// `<left> <op> <right>`
    pub fn new(left: Expr, op: Cmp, right: Expr) -> Self {
        Self {
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }
}

impl From<CmpExpr> for Expr {
    fn from(value: CmpExpr) -> Self {
        Self::Cmp(value)
    }
}

impl Node for CmpExpr {
    fn name(&self) -> &str {
        "Expr::Binary::Cmp"
    }
}

impl std::fmt::Display for CmpExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Spanned for CmpExpr {
    fn span(&self) -> Span {
        Span::join(self.left.span(), self.right.span())
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
        let mut cursor = Span::from_bytes(b"a >= b").cursor();
        let value = parser.parse_expr(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "a >= b");
        debug_assert!(value.is_cmp());
        Ok(())
    }
}
