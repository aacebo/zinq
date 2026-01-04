use zinq_parse::{Parse, Parser, Peek, Span};
use zinq_token::{Comma, LParen, Punctuated, RParen, TokenParser};

use crate::{Node, Visitor, expr::Expr};

///
/// ## Invoke Expression
/// `do_stuff(arg1, arg2 = "test", ...)`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvokeExpr {
    pub span: Span,
    pub target: Box<Expr>,
    pub left_paren: LParen,
    pub args: Punctuated<Expr, Comma>,
    pub right_paren: RParen,
}

impl From<InvokeExpr> for Expr {
    fn from(value: InvokeExpr) -> Self {
        Self::Invoke(value)
    }
}

impl Node for InvokeExpr {
    fn name(&self) -> &str {
        "Syntax::Expr::Invoke"
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for InvokeExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for InvokeExpr {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse<TokenParser> for InvokeExpr {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        let target = parser.parse_as::<Expr>(cursor)?;
        let left_paren = parser.parse_as::<LParen>(cursor)?;
        let args = parser.parse_as::<Punctuated<Expr, Comma>>(cursor)?;
        let right_paren = parser.parse_as::<RParen>(cursor)?;

        Ok(Self {
            span: Span::from_bounds(target.span(), right_paren.span()),
            target: Box::new(target),
            left_paren,
            args,
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
    use zinq_parse::{Parser, Span};

    use crate::{TokenParser, expr::InvokeExpr};

    #[test]
    fn should_parse() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(b"stuff(a, b = \"test\")").cursor();
        let value = parser.parse_as::<InvokeExpr>(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "stuff(a, b = \"test\")");
        debug_assert_eq!(value.args.len(), 2);

        debug_assert!(value.args.get(0).unwrap().0.is_get());
        debug_assert_eq!(value.args.get(0).unwrap().0.to_string(), "a");

        debug_assert!(value.args.get(1).unwrap().0.is_assign());
        debug_assert_eq!(value.args.get(1).unwrap().0.to_string(), "b = \"test\"");

        Ok(())
    }
}
