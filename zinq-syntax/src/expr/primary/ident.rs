use zinq_parse::{Parse, Peek, Span};
use zinq_token::Ident;

use crate::{Node, Visitor, expr::Expr};

///
/// ## Ident Expression
/// `my_var`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentExpr {
    pub span: Span,
    pub name: Ident,
}

impl From<IdentExpr> for Expr {
    fn from(value: IdentExpr) -> Self {
        Self::Ident(value)
    }
}

impl Node for IdentExpr {
    fn name(&self) -> &str {
        "Syntax::Expr::Primary::Ident"
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for IdentExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek for IdentExpr {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser.peek::<Ident>(cursor).unwrap_or(false))
    }
}

impl Parse for IdentExpr {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let name = parser.parse::<Ident>(cursor)?;

        Ok(Self {
            span: name.span().clone(),
            name,
        })
    }

    fn span(&self) -> &Span {
        &self.span
    }
}
