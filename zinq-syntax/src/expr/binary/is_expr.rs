use zinq_parse::{Span, Spanned};
use zinq_token::Is;

use crate::{Node, expr::Expr, ty::Type};

///
/// ## Is Expression
/// ```
/// if <left> is <type> {
///     ...
/// }
/// ```
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IsExpr {
    pub left: Box<Expr>,
    pub keyword: Is,
    pub ty: Type,
}

impl IsExpr {
    /// `<left> is <ty>`
    pub fn new(left: Expr, keyword: Is, ty: Type) -> Self {
        Self {
            left: Box::new(left),
            keyword,
            ty,
        }
    }
}

impl From<IsExpr> for Expr {
    fn from(value: IsExpr) -> Self {
        Self::Is(value)
    }
}

impl Node for IsExpr {
    fn name(&self) -> &str {
        "Expr::Binary::Is"
    }

    fn accept<V: crate::Visitor>(&self, visitor: &mut V) {
        visitor.visit_is_expr(self);
        self.left.accept(visitor);
        self.ty.accept(visitor);
    }
}

impl std::fmt::Display for IsExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Spanned for IsExpr {
    fn span(&self) -> Span {
        Span::join(self.left.span(), self.ty.span())
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
        let mut cursor = Span::from_bytes(b"a is &string").cursor();
        let value = parser.parse_expr(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "a is &string");
        debug_assert!(value.is_is());
        Ok(())
    }
}
