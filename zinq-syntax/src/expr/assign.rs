use zinq_parse::{Parse, Parser, Peek, Span};
use zinq_token::{Eq, Ident, TokenParser};

use crate::{Node, Visitor, expr::Expr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssignExpr {
    pub span: Span,
    pub name: Ident,
    pub eq: Eq,
    pub expr: Box<Expr>,
}

impl From<AssignExpr> for Expr {
    fn from(value: AssignExpr) -> Self {
        Self::Assign(value)
    }
}

impl Node for AssignExpr {
    fn name(&self) -> &str {
        "Syntax::Expr::Assign"
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for AssignExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for AssignExpr {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse<TokenParser> for AssignExpr {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        let name = parser.parse_as::<Ident>(cursor)?;
        let eq = parser.parse_as::<Eq>(cursor)?;
        let expr = parser.parse_as::<Expr>(cursor)?;

        Ok(Self {
            span: Span::from_bounds(name.span(), expr.span()),
            name,
            eq,
            expr: Box::new(expr),
        })
    }

    fn span(&self) -> &Span {
        &self.span
    }
}
