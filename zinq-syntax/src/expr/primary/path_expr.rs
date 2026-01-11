use zinq_parse::{Parse, Peek, Span, Spanned};

use crate::{Node, Path, Visitor, expr::Expr};

///
/// ## Path Expression
/// `my_var`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathExpr {
    pub path: Path,
}

impl From<PathExpr> for Expr {
    fn from(value: PathExpr) -> Self {
        Self::Path(value)
    }
}

impl Node for PathExpr {
    fn name(&self) -> &str {
        "Syntax::Expr::Primary::Path"
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for PathExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Peek for PathExpr {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser.peek::<Path>(cursor).unwrap_or(false))
    }
}

impl Parse for PathExpr {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let path = parser.parse::<Path>(cursor)?;
        Ok(Self { path })
    }
}

impl Spanned for PathExpr {
    fn span(&self) -> Span {
        self.path.span()
    }
}
