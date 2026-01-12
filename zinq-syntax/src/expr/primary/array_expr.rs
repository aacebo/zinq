use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::{Comma, LBracket, Punctuated, RBracket, Suffixed};

use crate::{Node, expr::Expr};

///
/// ## Array Expression
/// ```
/// [a, b, c]
/// ```
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayExpr {
    pub left_bracket: LBracket,
    pub items: Punctuated<Expr, Comma>,
    pub right_bracket: RBracket,
}

impl From<ArrayExpr> for Expr {
    fn from(value: ArrayExpr) -> Self {
        Self::Array(value)
    }
}

impl Node for ArrayExpr {
    fn name(&self) -> &str {
        "Expr::Primary::Array"
    }
}

impl std::fmt::Display for ArrayExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Spanned for ArrayExpr {
    fn span(&self) -> Span {
        Span::join(self.left_bracket.span(), self.right_bracket.span())
    }
}

impl Peek for ArrayExpr {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser
            .peek::<Suffixed<Suffixed<LBracket, Expr>, Comma>>(cursor)
            .unwrap_or(false))
    }
}

impl Parse for ArrayExpr {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let left_bracket = parser.parse::<LBracket>(cursor)?;
        let items = parser.parse::<Punctuated<Expr, Comma>>(cursor)?;
        let right_bracket = parser.parse::<RBracket>(cursor)?;

        Ok(Self {
            left_bracket,
            items,
            right_bracket,
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
        let mut cursor = Span::from_bytes(b"[1, 2, (1 + 2)]").cursor();
        let value = parser.parse_expr(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "[1, 2, (1 + 2)]");
        debug_assert!(value.is_array());
        debug_assert_eq!(value.as_array().items.len(), 3);
        Ok(())
    }
}
