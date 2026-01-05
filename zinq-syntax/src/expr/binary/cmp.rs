use zinq_parse::{Parse, Peek, Span};
use zinq_token::Cmp;

use crate::{
    Node, Visitor,
    expr::{BinaryExpr, Expr},
};

///
/// ## Cmp Expression
/// `<left> <op> <right>`
/// ### Example
/// `<left> >= <right>`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmpExpr {
    pub span: Span,
    pub left: Box<Expr>,
    pub op: Cmp,
    pub right: Box<Expr>,
}

impl From<CmpExpr> for BinaryExpr {
    fn from(value: CmpExpr) -> Self {
        Self::Cmp(value)
    }
}

impl Node for CmpExpr {
    fn name(&self) -> &str {
        "Syntax::Expr::Binary::Cmp"
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for CmpExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek for CmpExpr {
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

impl Parse for CmpExpr {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let left = parser.parse::<Expr>(cursor)?;
        let op = parser.parse::<Cmp>(cursor)?;
        let right = parser.parse::<Expr>(cursor)?;

        Ok(Self {
            span: Span::from_bounds(left.span(), right.span()),
            left: Box::new(left),
            op,
            right: Box::new(right),
        })
    }

    fn span(&self) -> &Span {
        &self.span
    }
}
