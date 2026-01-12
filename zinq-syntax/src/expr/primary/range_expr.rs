use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::{DotDot, LBracket, RBracket};

use crate::{Node, Visitor, expr::Expr};

///
/// ## Range Expression
/// - `[..<end>]`
/// - `[<start>..]`
/// - `[<start>..<end>]`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RangeExpr {
    pub left_bracket: LBracket,
    pub start: Option<Box<Expr>>,
    pub dots: DotDot,
    pub end: Option<Box<Expr>>,
    pub right_bracket: RBracket,
}

impl From<RangeExpr> for Expr {
    fn from(value: RangeExpr) -> Self {
        Self::Range(value)
    }
}

impl Node for RangeExpr {
    fn name(&self) -> &str {
        "Syntax::Expr::Primary::Range"
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for RangeExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Peek for RangeExpr {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse for RangeExpr {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let left_bracket = parser.parse::<LBracket>(cursor)?;
        let start = parser.parse::<Option<Box<Expr>>>(cursor)?;
        let dots = parser.parse::<DotDot>(cursor)?;
        let end = parser.parse::<Option<Box<Expr>>>(cursor)?;
        let right_bracket = parser.parse::<RBracket>(cursor)?;

        Ok(Self {
            left_bracket,
            start,
            dots,
            end,
            right_bracket,
        })
    }
}

impl Spanned for RangeExpr {
    fn span(&self) -> Span {
        Span::join(self.left_bracket.span(), self.right_bracket.span())
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
        let mut cursor = Span::from_bytes(b"[0..3]").cursor();
        let value = parser.parse_expr(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "[0..3]");
        debug_assert!(value.is_range());
        Ok(())
    }
}
