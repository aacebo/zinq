use zinq_parse::{Parse, Peek, Span};
use zinq_token::{LParen, RParen};

use crate::{Node, Visitor, expr::Expr};

///
/// ## Group Expression
/// `(...)`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupExpr {
    pub span: Span,
    pub left_paren: LParen,
    pub inner: Box<Expr>,
    pub right_paren: RParen,
}

impl From<GroupExpr> for Expr {
    fn from(value: GroupExpr) -> Self {
        Self::Group(value)
    }
}

impl Node for GroupExpr {
    fn name(&self) -> &str {
        "Syntax::Expr::Primary::Group"
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for GroupExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek for GroupExpr {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser.peek::<LParen>(cursor).unwrap_or(false))
    }
}

impl Parse for GroupExpr {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let left_paren = parser.parse::<LParen>(cursor)?;
        let inner = parser.parse::<Box<Expr>>(cursor)?;
        let right_paren = parser.parse::<RParen>(cursor)?;

        Ok(Self {
            span: Span::from_bounds(left_paren.span(), right_paren.span()),
            left_paren,
            inner,
            right_paren,
        })
    }

    fn span(&self) -> &Span {
        &self.span
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
        let mut cursor = Span::from_bytes(b"(test == b'h')").cursor();
        let value = parser.parse_expr(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "(test == b'h')");
        Ok(())
    }

    #[test]
    fn should_parse_and_or() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"a != b || (b == c && c == d)").cursor();
        let value = parser.parse_expr(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "a != b || (b == c && c == d)");
        debug_assert!(value.is_or());
        debug_assert!(value.as_logical().left.is_cmp());
        debug_assert!(value.as_logical().right.is_group());
        Ok(())
    }
}
