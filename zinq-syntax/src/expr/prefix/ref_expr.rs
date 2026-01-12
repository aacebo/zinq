use zinq_parse::{Span, Spanned};
use zinq_token::{And, Mut};

use crate::{Node, expr::Expr};

///
/// ## Reference Expression
/// `&<expr>` or `&mut <expr>`
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RefExpr {
    pub and: And,
    pub mutable: Option<Mut>,
    pub right: Box<Expr>,
}

impl RefExpr {
    /// `&<right>`
    pub fn new(and: And, mutable: Option<Mut>, right: Expr) -> Self {
        Self {
            and,
            mutable,
            right: Box::new(right),
        }
    }
}

impl From<RefExpr> for Expr {
    fn from(value: RefExpr) -> Self {
        Self::Ref(value)
    }
}

impl Node for RefExpr {
    fn name(&self) -> &str {
        "Expr::Prefix::Ref"
    }
}

impl std::fmt::Display for RefExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Spanned for RefExpr {
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

    #[test]
    fn should_parse_mut_ref() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"&mut a").cursor();
        let value = parser.parse_expr(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "&mut a");
        Ok(())
    }
}
