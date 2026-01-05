use zinq_parse::{Parse, Peek, Span};
use zinq_token::Not;

use crate::{
    Node, Visitor,
    expr::{Expr, UnaryExpr},
};

///
/// ## Not Expression
/// `!(a == true)`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotExpr {
    pub span: Span,
    pub not: Not,
    pub right: Box<Expr>,
}

impl From<NotExpr> for UnaryExpr {
    fn from(value: NotExpr) -> Self {
        Self::Not(value)
    }
}

impl Node for NotExpr {
    fn name(&self) -> &str {
        "Syntax::Expr::Unary::Not"
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for NotExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek for NotExpr {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser.peek::<Not>(cursor).unwrap_or(false))
    }
}

impl Parse for NotExpr {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let not = parser.parse::<Not>(cursor)?;
        let right = parser.parse::<Box<Expr>>(cursor)?;

        Ok(Self {
            span: Span::from_bounds(not.span(), right.span()),
            not,
            right,
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

    use crate::expr::NotExpr;

    #[test]
    fn should_parse_not() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"!b").cursor();
        let value = parser.parse::<NotExpr>(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "!b");
        Ok(())
    }

    #[test]
    fn should_parse_not_of_group() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"!(a)").cursor();
        let value = parser.parse::<NotExpr>(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "!(a)");
        Ok(())
    }
}
