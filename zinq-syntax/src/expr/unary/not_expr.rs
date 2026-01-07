use zinq_parse::{Parse, Span};
use zinq_token::Not;

use crate::{Node, Visitor, expr::Expr};

///
/// ## Not Expression
/// `!(a == true)`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotExpr {
    pub span: Span,
    pub not: Not,
    pub right: Box<Expr>,
}

impl NotExpr {
    /// `!<right>`
    pub fn new(not: Not, right: Expr) -> Self {
        Self {
            span: Span::from_bounds(not.span(), right.span()),
            not,
            right: Box::new(right),
        }
    }

    pub fn span(&self) -> &Span {
        &self.span
    }
}

impl From<NotExpr> for Expr {
    fn from(value: NotExpr) -> Self {
        Self::Not(value)
    }
}

impl Node for NotExpr {
    fn name(&self) -> &str {
        "Syntax::Expr::Unary::Not"
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for NotExpr {
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
