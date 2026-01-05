use zinq_parse::{Parse, Peek, Span};
use zinq_token::{Dot, Ident};

use crate::{
    Node, Visitor,
    expr::{Expr, PrimaryExpr},
};

///
/// ## Get Field Expression
/// `my.field`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetFieldExpr {
    pub span: Span,
    pub target: Box<Expr>,
    pub dot: Dot,
    pub name: Ident,
}

impl From<GetFieldExpr> for PrimaryExpr {
    fn from(value: GetFieldExpr) -> Self {
        Self::GetField(value)
    }
}

impl Node for GetFieldExpr {
    fn name(&self) -> &str {
        "Syntax::Expr::Primary::GetField"
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for GetFieldExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek for GetFieldExpr {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse for GetFieldExpr {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let target = parser.parse::<Expr>(cursor)?;
        let dot = parser.parse::<Dot>(cursor)?;
        let name = parser.parse::<Ident>(cursor)?;

        Ok(Self {
            span: Span::from_bounds(target.span(), name.span()),
            target: Box::new(target),
            dot,
            name,
        })
    }

    fn span(&self) -> &Span {
        &self.span
    }
}
