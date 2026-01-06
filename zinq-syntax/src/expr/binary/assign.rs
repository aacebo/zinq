use zinq_parse::{Parse, Span};
use zinq_token::Eq;

use crate::{Node, Visitor, expr::Expr};

///
/// ## Assign Expression
/// `message = (...)`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssignExpr {
    pub span: Span,
    pub left: Box<Expr>,
    pub eq: Eq,
    pub right: Box<Expr>,
}

impl AssignExpr {
    /// `<left> = <right>`
    pub fn new(left: Expr, eq: Eq, right: Expr) -> Self {
        Self {
            span: Span::from_bounds(left.span(), right.span()),
            left: Box::new(left),
            eq,
            right: Box::new(right),
        }
    }

    pub fn span(&self) -> &Span {
        &self.span
    }
}

impl From<AssignExpr> for Expr {
    fn from(value: AssignExpr) -> Self {
        Self::Assign(value)
    }
}

impl Node for AssignExpr {
    fn name(&self) -> &str {
        "Syntax::Expr::Binary::Assign"
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for AssignExpr {
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
        let mut cursor = Span::from_bytes(b"a = b'h'").cursor();
        let value = parser.parse_expr(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "a = b'h'");

        Ok(())
    }
}
