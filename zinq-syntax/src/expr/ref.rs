use zinq_parse::{Parse, Parser, Peek, Span};
use zinq_token::{And, TokenParser};

use crate::{Node, Visitor, expr::Expr};

///
/// ## Ref Expression
/// `&a`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefExpr {
    pub span: Span,
    pub and: And,
    pub expr: Box<Expr>,
}

impl From<RefExpr> for Expr {
    fn from(value: RefExpr) -> Self {
        Self::Ref(value)
    }
}

impl Node for RefExpr {
    fn name(&self) -> &str {
        "Syntax::Expr::Ref"
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for RefExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for RefExpr {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse<TokenParser> for RefExpr {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        let and = parser.parse_as::<And>(cursor)?;
        let expr = parser.parse_as::<Box<Expr>>(cursor)?;

        Ok(Self {
            span: Span::from_bounds(and.span(), expr.span()),
            and,
            expr,
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

    use crate::{TokenParser, expr::RefExpr};

    #[test]
    fn should_parse() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(b"&a").cursor();
        let value = parser.parse_as::<RefExpr>(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "&a");
        debug_assert!(value.expr.is_get());

        Ok(())
    }
}
