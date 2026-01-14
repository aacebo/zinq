use zinq_parse::{Span, Spanned};
use zinq_token::DotDot;

use crate::{Syntax, expr::Expr};

///
/// ## Range Expression
/// - `..<end>`
/// - `<start>..`
/// - `<start>..<end>`
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RangeExpr {
    pub start: Option<Box<Expr>>,
    pub dots: DotDot,
    pub end: Option<Box<Expr>>,
}

impl From<RangeExpr> for Expr {
    fn from(value: RangeExpr) -> Self {
        Self::Range(value)
    }
}

impl Syntax for RangeExpr {
    fn name(&self) -> &str {
        "Expr::Infix::Range"
    }

    fn accept<V: crate::Visitor>(&self, visitor: &mut V) {
        visitor.visit_range_expr(self);

        if let Some(start) = &self.start {
            start.accept(visitor);
        }

        if let Some(end) = &self.end {
            end.accept(visitor);
        }
    }
}

impl std::fmt::Display for RangeExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Spanned for RangeExpr {
    fn span(&self) -> Span {
        let mut start = self.dots.span();
        let mut end = self.dots.span();

        if let Some(v) = &self.start {
            start = v.span();
        }

        if let Some(v) = &self.end {
            end = v.span();
        }

        Span::join(start, end)
    }
}

#[cfg(test)]
mod tests {
    use zinq_error::Result;
    use zinq_parse::Span;

    use crate::expr::ExprParser;

    #[test]
    fn should_parse_full() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"0..3").cursor();
        let expr = parser.parse_expr(&mut cursor)?;

        debug_assert_eq!(expr.to_string(), "0..3");
        debug_assert!(expr.is_range());

        Ok(())
    }

    #[test]
    fn should_parse_prefix() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"3..").cursor();
        let expr = parser.parse_expr(&mut cursor)?;

        debug_assert_eq!(expr.to_string(), "3..");
        debug_assert!(expr.is_range());

        Ok(())
    }

    #[test]
    fn should_parse_suffix() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"..5").cursor();
        let expr = parser.parse_expr(&mut cursor)?;

        debug_assert_eq!(expr.to_string(), "..5");
        debug_assert!(expr.is_range());

        Ok(())
    }
}
