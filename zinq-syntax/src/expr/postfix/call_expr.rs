use zinq_parse::{Parse, Span};
use zinq_token::{Comma, LParen, Punctuated, RParen};

use crate::{Node, Visitor, expr::Expr};

///
/// ## Call Expression
/// `do_stuff(arg1, arg2 = "test", ...)`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallExpr {
    pub span: Span,
    pub target: Box<Expr>,
    pub left_paren: LParen,
    pub args: Punctuated<Expr, Comma>,
    pub right_paren: RParen,
}

impl CallExpr {
    /// `<target>(<arg1>, <arg2>, ...)`
    pub fn new(
        target: Expr,
        left_paren: LParen,
        args: Punctuated<Expr, Comma>,
        right_paren: RParen,
    ) -> Self {
        Self {
            span: Span::from_bounds(target.span(), right_paren.span()),
            target: Box::new(target),
            left_paren,
            args,
            right_paren,
        }
    }

    pub fn span(&self) -> &Span {
        &self.span
    }
}

impl From<CallExpr> for Expr {
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

#[cfg(test)]
mod test {
    use zinq_error::Result;
    use zinq_parse::Span;

    use crate::expr::ExprParser;

    #[test]
    fn should_parse() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"stuff(a, b = \"test\")").cursor();
        let value = parser.parse_expr(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "stuff(a, b = \"test\")");

        Ok(())
    }
}
