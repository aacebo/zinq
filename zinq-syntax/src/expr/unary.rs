use zinq_error::BAD_ARGUMENTS;
use zinq_parse::{Parse, Parser, Peek, Span};
use zinq_token::{And, Not, Punct, TokenParser};

use crate::{Node, Visitor, expr::Expr};

///
/// ## Unary Expression
/// `!(a == true)`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnaryExpr {
    pub span: Span,
    pub op: Punct,
    pub right: Box<Expr>,
}

impl From<UnaryExpr> for Expr {
    fn from(value: UnaryExpr) -> Self {
        Self::Unary(value)
    }
}

impl Node for UnaryExpr {
    fn name(&self) -> &str {
        "Syntax::Expr::Unary"
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for UnaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for UnaryExpr {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse<TokenParser> for UnaryExpr {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        if !parser.peek_as::<Not>(cursor).unwrap_or(false)
            && !parser.peek_as::<And>(cursor).unwrap_or(false)
        {
            return Err(cursor.error(BAD_ARGUMENTS, "expected '!' or '&'"));
        }

        let op = parser.parse_as::<Punct>(cursor)?;
        let right = parser.parse_as::<Expr>(cursor)?;

        Ok(Self {
            span: Span::from_bounds(op.span(), right.span()),
            op,
            right: Box::new(right),
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

    use crate::{TokenParser, expr::UnaryExpr};

    #[test]
    fn should_parse_ref() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(b"&b").cursor();
        let value = parser.parse_as::<UnaryExpr>(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "&b");
        debug_assert!(value.op.is_and());
        debug_assert!(value.right.is_get());

        Ok(())
    }

    #[test]
    fn should_parse_ref_of_group() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(b"&(a)").cursor();
        let value = parser.parse_as::<UnaryExpr>(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "&(a)");
        debug_assert!(value.op.is_and());
        debug_assert!(value.right.is_group());

        Ok(())
    }

    #[test]
    fn should_parse_not() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(b"!b").cursor();
        let value = parser.parse_as::<UnaryExpr>(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "!b");
        debug_assert!(value.op.is_not());
        debug_assert!(value.right.is_get());

        Ok(())
    }

    #[test]
    fn should_parse_not_of_group() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(b"!(a)").cursor();
        let value = parser.parse_as::<UnaryExpr>(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "!(a)");
        debug_assert!(value.op.is_not());
        debug_assert!(value.right.is_group());

        Ok(())
    }
}
