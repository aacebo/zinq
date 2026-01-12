use zinq_parse::{Span, Spanned};
use zinq_token::{Colon, Question};

use crate::{Node, expr::Expr};

///
/// ## If Expression
/// `<cond> ? <then> : <else>`
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IfExpr {
    pub cond: Box<Expr>,
    pub question: Question,
    pub then_expr: Box<Expr>,
    pub colon: Colon,
    pub else_expr: Box<Expr>,
}

impl From<IfExpr> for Expr {
    fn from(value: IfExpr) -> Self {
        Self::If(value)
    }
}

impl Node for IfExpr {
    fn name(&self) -> &str {
        "Expr::If"
    }
}

impl std::fmt::Display for IfExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Spanned for IfExpr {
    fn span(&self) -> zinq_parse::Span {
        Span::join(self.cond.span(), self.else_expr.span())
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
        let mut cursor = Span::from_bytes(b"a < 5 ? 1 : -1").cursor();
        let expr = parser.parse_expr(&mut cursor)?;

        debug_assert!(expr.is_if());
        debug_assert_eq!(expr.to_string(), "a < 5 ? 1 : -1");
        Ok(())
    }

    #[test]
    fn should_parse_nested() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"a < 5 ? a < 3 ? 2 : 1 : -1").cursor();
        let expr = parser.parse_expr(&mut cursor)?;

        debug_assert!(expr.is_if());
        debug_assert!(expr.as_if().then_expr.is_if());
        debug_assert_eq!(expr.to_string(), "a < 5 ? a < 3 ? 2 : 1 : -1");
        Ok(())
    }
}
