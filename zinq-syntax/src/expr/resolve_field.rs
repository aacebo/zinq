use zinq_parse::{Parse, Parser, Peek, Span};
use zinq_token::{Dot, Ident, TokenParser};

use crate::{Node, Visitor, expr::Expr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolveFieldExpr {
    pub span: Span,
    pub target: Box<Expr>,
    pub dot: Dot,
    pub field: Ident,
}

impl From<ResolveFieldExpr> for Expr {
    fn from(value: ResolveFieldExpr) -> Self {
        Self::ResolveField(value)
    }
}

impl Node for ResolveFieldExpr {
    fn name(&self) -> &str {
        "Syntax::Expr::Resolve::Field"
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for ResolveFieldExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for ResolveFieldExpr {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse<TokenParser> for ResolveFieldExpr {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        let target = parser.parse_as::<Expr>(cursor)?;
        let dot = parser.parse_as::<Dot>(cursor)?;
        let field = parser.parse_as::<Ident>(cursor)?;

        Ok(Self {
            span: Span::from_bounds(target.span(), field.span()),
            target: Box::new(target),
            dot,
            field,
        })
    }

    fn span(&self) -> &Span {
        &self.span
    }
}
