use zinq_parse::{Span, Spanned};
use zinq_token::{LBracket, RBracket};

use crate::{Node, expr::Expr};

///
/// ## Index Expression
/// `arr[0]`
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IndexExpr {
    pub target: Box<Expr>,
    pub left_bracket: LBracket,
    pub index: Box<Expr>,
    pub right_bracket: RBracket,
}

impl From<IndexExpr> for Expr {
    fn from(value: IndexExpr) -> Self {
        Self::Index(value)
    }
}

impl Node for IndexExpr {
    fn name(&self) -> &str {
        "Expr::Postfix::Index"
    }
}

impl std::fmt::Display for IndexExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Spanned for IndexExpr {
    fn span(&self) -> Span {
        Span::join(self.target.span(), self.right_bracket.span())
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
        let mut cursor = Span::from_bytes(b"a.b[3]").cursor();
        let value = parser.parse_expr(&mut cursor)?;

        debug_assert!(value.is_index());
        debug_assert_eq!(value.to_string(), "a.b[3]");
        debug_assert_eq!(value.as_index().index.to_string(), "3");

        Ok(())
    }
}
