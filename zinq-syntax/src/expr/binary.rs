use zinq_parse::{Parse, Parser, Peek, Span};
use zinq_token::{Punct, TokenParser};

use crate::{Node, Visitor, expr::Expr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryExpr {
    pub span: Span,
    pub left: Box<Expr>,
    pub op: Punct,
    pub right: Box<Expr>,
}

impl From<BinaryExpr> for Expr {
    fn from(value: BinaryExpr) -> Self {
        Self::Binary(value)
    }
}

impl Node for BinaryExpr {
    fn name(&self) -> &str {
        "Syntax::Expr::Binary"
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for BinaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for BinaryExpr {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse<TokenParser> for BinaryExpr {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        let left = parser.parse_as::<Expr>(cursor)?;
        let op = parser.parse_as::<Punct>(cursor)?;
        let right = parser.parse_as::<Expr>(cursor)?;

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
