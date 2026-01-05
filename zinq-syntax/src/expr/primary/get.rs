use zinq_parse::{Parse, Parser, Peek, Span};
use zinq_token::{Ident, TokenParser};

use crate::{Node, Visitor, expr::PrimaryExpr};

///
/// ## Get Expression
/// `my_var`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetExpr {
    pub span: Span,
    pub name: Ident,
}

impl From<GetExpr> for PrimaryExpr {
    fn from(value: GetExpr) -> Self {
        Self::Get(value)
    }
}

impl Node for GetExpr {
    fn name(&self) -> &str {
        "Syntax::Expr::Primary::Get"
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for GetExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for GetExpr {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        Ok(parser.peek_as::<Ident>(cursor).unwrap_or(false))
    }
}

impl Parse<TokenParser> for GetExpr {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        let name = parser.parse_as::<Ident>(cursor)?;

        Ok(Self {
            span: name.span().clone(),
            name,
        })
    }

    fn span(&self) -> &Span {
        &self.span
    }
}
