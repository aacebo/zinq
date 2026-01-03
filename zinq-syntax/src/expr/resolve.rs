use zinq_parse::{Parse, Parser, Peek, Span};
use zinq_token::{Ident, TokenParser};

use crate::{Node, Visitor, expr::Expr};

///
/// ## Resolve Expression
/// `my_var`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolveExpr {
    pub span: Span,
    pub name: Ident,
}

impl From<ResolveExpr> for Expr {
    fn from(value: ResolveExpr) -> Self {
        Self::Resolve(value)
    }
}

impl Node for ResolveExpr {
    fn name(&self) -> &str {
        "Syntax::Expr::Resolve"
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for ResolveExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for ResolveExpr {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse<TokenParser> for ResolveExpr {
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
