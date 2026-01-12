mod arm;

pub use arm::*;

use zinq_parse::{Span, Spanned};
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
        "Expr::Match"
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

    #[test]
    fn should_parse_struct_pattern() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(
            b"match self {
            A { a } => \"A\",
            B { b } => \"B\",
        }",
        )
        .cursor();

        let expr = parser.parse_expr(&mut cursor)?;

        debug_assert!(expr.is_match());
        debug_assert_eq!(expr.as_match().arms.len(), 2);
        debug_assert_eq!(
            expr.to_string(),
            "match self {
            A { a } => \"A\",
            B { b } => \"B\",
        }"
        );

        Ok(())
    }

    #[test]
    fn should_parse_type_pattern() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(
            b"match typeof(self) {
            std::string::String => \"STRING\",
            bool => \"BOOLEAN\",
        }",
        )
        .cursor();

        let expr = parser.parse_expr(&mut cursor)?;

        debug_assert!(expr.is_match());
        debug_assert_eq!(expr.as_match().arms.len(), 2);
        debug_assert_eq!(
            expr.to_string(),
            "match typeof(self) {
            std::string::String => \"STRING\",
            bool => \"BOOLEAN\",
        }"
        );

        Ok(())
    }

    #[test]
    fn should_parse_group_pattern() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(
            b"match typeof(self) {
            (std::string::String | bool) => \"STRING OR BOOL\",
        }",
        )
        .cursor();

        let expr = parser.parse_expr(&mut cursor)?;

        debug_assert!(expr.is_match());
        debug_assert_eq!(expr.as_match().arms.len(), 1);
        debug_assert!(
            expr.as_match()
                .arms
                .first()
                .unwrap()
                .value()
                .pattern
                .is_group()
        );
        debug_assert_eq!(
            expr.to_string(),
            "match typeof(self) {
            (std::string::String | bool) => \"STRING OR BOOL\",
        }"
        );

        Ok(())
    }

    #[test]
    fn should_parse_tuple_pattern() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(
            b"match &value {
            (a, b, c) => \"TUPLE\",
        }",
        )
        .cursor();

        let expr = parser.parse_expr(&mut cursor)?;

        debug_assert!(expr.is_match());
        debug_assert_eq!(expr.as_match().arms.len(), 1);
        debug_assert!(
            expr.as_match()
                .arms
                .first()
                .unwrap()
                .value()
                .pattern
                .is_tuple()
        );
        debug_assert_eq!(
            expr.to_string(),
            "match &value {
            (a, b, c) => \"TUPLE\",
        }"
        );

        Ok(())
    }
}
