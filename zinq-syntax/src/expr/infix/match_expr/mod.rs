mod arm;

use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::{Comma, LBrace, Match, Punctuated, RBrace};

use crate::{Node, expr::Expr};

///
/// ## Match Expression
/// ```
/// match <expr> {
///     <path> => <expr>,
/// }
/// ```
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchExpr {
    pub keyword: Match,
    pub expr: Box<Expr>,
    pub left_brace: LBrace,
    pub arms: Punctuated<arm::Arm, Comma>,
    pub right_brace: RBrace,
}

impl From<MatchExpr> for Expr {
    fn from(value: MatchExpr) -> Self {
        Self::Match(value)
    }
}

impl Node for MatchExpr {
    fn name(&self) -> &str {
        "Syntax::Expr::Match"
    }

    fn accept<V: crate::Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for MatchExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Spanned for MatchExpr {
    fn span(&self) -> zinq_parse::Span {
        Span::join(self.keyword.span(), self.right_brace.span())
    }
}

impl Peek for MatchExpr {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser.peek::<Match>(cursor).unwrap_or(false))
    }
}

impl Parse for MatchExpr {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let keyword = parser.parse::<Match>(cursor)?;
        let expr = parser.parse::<Box<Expr>>(cursor)?;
        let left_brace = parser.parse::<LBrace>(cursor)?;
        let arms = parser.parse::<Punctuated<arm::Arm, Comma>>(cursor)?;
        let right_brace = parser.parse::<RBrace>(cursor)?;

        Ok(Self {
            keyword,
            expr,
            left_brace,
            arms,
            right_brace,
        })
    }
}

#[cfg(test)]
mod test {
    use zinq_error::Result;
    use zinq_parse::Span;

    use crate::expr::ExprParser;

    #[test]
    fn should_parse_literal_pattern() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(
            b"match self {
            true => \"TRUE\",
            false => \"FALSE\",
        }",
        )
        .cursor();

        let expr = parser.parse_expr(&mut cursor)?;

        debug_assert!(expr.is_match());
        debug_assert_eq!(expr.as_match().arms.len(), 2);
        debug_assert_eq!(
            expr.to_string(),
            "match self {
            true => \"TRUE\",
            false => \"FALSE\",
        }"
        );

        Ok(())
    }
}
