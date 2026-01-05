use zinq_parse::{Parse, Parser, Peek, Span};
use zinq_token::{Not, TokenParser};

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

impl Peek<TokenParser> for NotExpr {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        Ok(parser.peek_as::<Not>(cursor).unwrap_or(false))
    }
}

impl Parse<TokenParser> for NotExpr {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        let not = parser.parse_as::<Not>(cursor)?;
        let right = parser.parse_as::<Box<Expr>>(cursor)?;

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
    use zinq_parse::{Parser, Span};

    use crate::{TokenParser, expr::NotExpr};

    #[test]
    fn should_parse_not() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(b"!b").cursor();
        let value = parser.parse_as::<NotExpr>(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "!b");
        Ok(())
    }

    #[test]
    fn should_parse_not_of_group() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(b"!(a)").cursor();
        let value = parser.parse_as::<NotExpr>(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "!(a)");
        Ok(())
    }
}
