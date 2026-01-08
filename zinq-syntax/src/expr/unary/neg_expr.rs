use zinq_parse::{Span, Spanned};
use zinq_token::Minus;

use crate::{Node, Visitor, expr::Expr};

///
/// ## Negative Expression
/// `-100`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NegExpr {
    pub minus: Minus,
    pub right: Box<Expr>,
}

impl NegExpr {
    /// `-<right>`
    pub fn new(minus: Minus, right: Expr) -> Self {
        Self {
            minus,
            right: Box::new(right),
        }
    }
}

impl From<NegExpr> for Expr {
    fn from(value: NegExpr) -> Self {
        Self::Neg(value)
    }
}

impl Node for NegExpr {
    fn name(&self) -> &str {
        "Syntax::Expr::Unary::Neg"
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for NegExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Spanned for NegExpr {
    fn span(&self) -> Span {
        Span::join(self.minus.span(), self.right.span())
    }
}

#[cfg(test)]
mod test {
    use zinq_error::Result;
    use zinq_parse::Span;

    use crate::expr::ExprParser;

    #[test]
    fn should_parse_negative() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"-1.03").cursor();
        let value = parser.parse_expr(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "-1.03");
        Ok(())
    }
}
