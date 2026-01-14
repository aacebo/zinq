use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::{EqArrow, Suffixed};

use crate::{
    Syntax,
    expr::{Expr, ExprParser},
    pat::Pattern,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Arm {
    pub pattern: Pattern,
    pub arrow: EqArrow,
    pub body: Box<Expr>,
}

impl Syntax for Arm {
    fn name(&self) -> &str {
        "Expr::Match::Arm"
    }

    fn accept<V: crate::Visitor>(&self, visitor: &mut V) {
        visitor.visit_match_arm(self);
        self.body.accept(visitor);
    }
}

impl std::fmt::Display for Arm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Spanned for Arm {
    fn span(&self) -> zinq_parse::Span {
        Span::join(self.pattern.span(), self.body.span())
    }
}

impl Peek for Arm {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser
            .peek::<Suffixed<Pattern, EqArrow>>(cursor)
            .unwrap_or(false))
    }
}

impl Parse for Arm {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let pattern = parser.parse::<Pattern>(cursor)?;
        let arrow = parser.parse::<EqArrow>(cursor)?;
        let body = parser.parse_expr(cursor)?;

        Ok(Self {
            pattern,
            arrow,
            body: Box::new(body),
        })
    }
}
