use zinq_parse::{Parse, Parser, Peek, Span};
use zinq_token::{Punct, TokenParser};

use crate::{
    Node, Visitor,
    expr::{BinaryExpr, Expr},
};

///
/// ## Logical Expression
/// `&&` or `||`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicalExpr {
    pub span: Span,
    pub left: Box<Expr>,
    pub op: Punct,
    pub right: Box<Expr>,
}

impl From<LogicalExpr> for BinaryExpr {
    fn from(value: LogicalExpr) -> Self {
        Self::Logical(value)
    }
}

impl Node for LogicalExpr {
    fn name(&self) -> &str {
        "Syntax::Expr::Binary::Logical"
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for LogicalExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for LogicalExpr {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse<TokenParser> for LogicalExpr {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        let left = parser.parse_as::<Expr>(cursor)?;
        let op = parser.parse_as::<Punct>(cursor)?;
        let right = parser.parse_as::<Expr>(cursor)?;

        Ok(Self {
            span: Span::from_bounds(left.span(), right.span()),
            left: Box::new(left),
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

    use crate::{TokenParser, expr::LogicalExpr};

    #[test]
    fn should_parse() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(b"a || true").cursor();
        let value = parser.parse_as::<LogicalExpr>(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "a || true");
        debug_assert_eq!(value.left.to_string(), "a");

        debug_assert!(value.op.is_or_or());
        debug_assert_eq!(value.op.to_string(), "||");
        debug_assert_eq!(value.right.to_string(), "true");

        Ok(())
    }
}
