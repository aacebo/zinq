use zinq_parse::{Parse, Parser, Peek, Span};
use zinq_token::{And, TokenParser};

use crate::{
    Node, Visitor,
    expr::{Expr, UnaryExpr},
};

///
/// ## Addr Expression
/// `&var`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddrExpr {
    pub span: Span,
    pub and: And,
    pub right: Box<Expr>,
}

impl From<AddrExpr> for UnaryExpr {
    fn from(value: AddrExpr) -> Self {
        Self::Addr(value)
    }
}

impl Node for AddrExpr {
    fn name(&self) -> &str {
        "Syntax::Expr::Unary::Addr"
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for AddrExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for AddrExpr {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        Ok(parser.peek_as::<And>(cursor).unwrap_or(false))
    }
}

impl Parse<TokenParser> for AddrExpr {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        let and = parser.parse_as::<And>(cursor)?;
        let right = parser.parse_as::<Box<Expr>>(cursor)?;

        Ok(Self {
            span: Span::from_bounds(and.span(), right.span()),
            and,
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

    use crate::{TokenParser, expr::AddrExpr};

    #[test]
    fn should_parse_ref() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(b"&b").cursor();
        let value = parser.parse_as::<AddrExpr>(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "&b");
        Ok(())
    }

    #[test]
    fn should_parse_ref_of_group() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(b"&(a)").cursor();
        let value = parser.parse_as::<AddrExpr>(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "&(a)");
        Ok(())
    }
}
