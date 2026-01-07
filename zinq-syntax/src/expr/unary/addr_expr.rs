use zinq_parse::{Span, Spanned};
use zinq_token::And;

use crate::{Node, Visitor, expr::Expr};

///
/// ## Addr Expression
/// `&var`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddrExpr {
    pub and: And,
    pub right: Box<Expr>,
}

impl AddrExpr {
    /// `&<right>`
    pub fn new(and: And, right: Expr) -> Self {
        Self {
            and,
            right: Box::new(right),
        }
    }
}

impl From<AddrExpr> for Expr {
    fn from(value: AddrExpr) -> Self {
        Self::Addr(value)
    }
}

impl Node for AddrExpr {
    fn name(&self) -> &str {
        "Syntax::Expr::Unary::Addr"
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for AddrExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Spanned for AddrExpr {
    fn span(&self) -> Span {
        Span::join(self.and.span(), self.right.span())
    }
}

#[cfg(test)]
mod test {
    use zinq_error::Result;
    use zinq_parse::Span;

    use crate::expr::ExprParser;

    #[test]
    fn should_parse_ref() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"&b").cursor();
        let value = parser.parse_expr(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "&b");
        Ok(())
    }

    #[test]
    fn should_parse_ref_of_group() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"&(a)").cursor();
        let value = parser.parse_expr(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "&(a)");
        Ok(())
    }
}
