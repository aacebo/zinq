use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::{Comma, LParen, Punctuated, RParen, Suffixed};

use crate::{Node, expr::Expr};

///
/// ## Tuple Expression
/// ```
/// (a, b, c)
/// ```
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TupleExpr {
    pub left_paren: LParen,
    pub items: Punctuated<Expr, Comma>,
    pub right_paren: RParen,
}

impl From<TupleExpr> for Expr {
    fn from(value: TupleExpr) -> Self {
        Self::Tuple(value)
    }
}

impl Node for TupleExpr {
    fn name(&self) -> &str {
        "Expr::Primary::Tuple"
    }
}

impl std::fmt::Display for TupleExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Spanned for TupleExpr {
    fn span(&self) -> Span {
        Span::join(self.left_paren.span(), self.right_paren.span())
    }
}

impl Peek for TupleExpr {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser
            .peek::<Suffixed<Suffixed<LParen, Expr>, Comma>>(cursor)
            .unwrap_or(false))
    }
}

impl Parse for TupleExpr {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let left_paren = parser.parse::<LParen>(cursor)?;
        let items = parser.parse::<Punctuated<Expr, Comma>>(cursor)?;
        let right_paren = parser.parse::<RParen>(cursor)?;

        Ok(Self {
            left_paren,
            items,
            right_paren,
        }
        .into())
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
        let mut cursor = Span::from_bytes(b"(1, a, true)").cursor();
        let value = parser.parse_expr(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "(1, a, true)");
        debug_assert!(value.is_tuple());
        debug_assert_eq!(value.as_tuple().items.len(), 3);
        Ok(())
    }
}
