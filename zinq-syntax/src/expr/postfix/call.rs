use zinq_parse::{Parse, Parser, Peek, Span};
use zinq_token::{Comma, LParen, Punctuated, RParen, TokenParser};

use crate::{
    Node, Visitor,
    expr::{Expr, PostfixExpr, PrimaryExpr},
};

///
/// ## Call Expression
/// `do_stuff(arg1, arg2 = "test", ...)`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallExpr {
    pub span: Span,
    pub target: PrimaryExpr,
    pub left_paren: LParen,
    pub args: Punctuated<Expr, Comma>,
    pub right_paren: RParen,
}

impl From<CallExpr> for PostfixExpr {
    fn from(value: CallExpr) -> Self {
        Self::Call(value)
    }
}

impl Node for CallExpr {
    fn name(&self) -> &str {
        "Syntax::Expr::Postfix::Call"
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for CallExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for CallExpr {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse<TokenParser> for CallExpr {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        let target = parser.parse_as::<PrimaryExpr>(cursor)?;
        let left_paren = parser.parse_as::<LParen>(cursor)?;
        let args = parser.parse_as::<Punctuated<Expr, Comma>>(cursor)?;
        let right_paren = parser.parse_as::<RParen>(cursor)?;

        Ok(Self {
            span: Span::from_bounds(target.span(), right_paren.span()),
            target,
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

    use crate::{TokenParser, expr::CallExpr};

    #[test]
    fn should_parse() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(b"stuff(a, b = \"test\")").cursor();
        let value = parser.parse_as::<CallExpr>(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "stuff(a, b = \"test\")");
        debug_assert_eq!(value.args.len(), 2);
        debug_assert_eq!(value.args.get(0).unwrap().0.to_string(), "a");
        debug_assert_eq!(value.args.get(1).unwrap().0.to_string(), "b = \"test\"");

        Ok(())
    }
}
