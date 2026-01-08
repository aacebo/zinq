use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::EqArrow;

use crate::{expr::Expr, pat::Pattern};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arm {
    pub pattern: Pattern,
    pub arrow: EqArrow,
    pub body: Box<Expr>,
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
        Ok(parser.peek::<Pattern>(cursor).unwrap_or(false))
    }
}

impl Parse for Arm {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let pattern = parser.parse::<Pattern>(cursor)?;
        let arrow = parser.parse::<EqArrow>(cursor)?;
        let body = parser.parse::<Box<Expr>>(cursor)?;

        Ok(Self {
            pattern,
            arrow,
            body,
        })
    }
}
