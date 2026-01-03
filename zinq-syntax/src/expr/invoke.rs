use zinq_parse::{Parse, Parser, Peek, Span};
use zinq_token::{Comma, LParen, Punctuated, RParen, TokenParser};

use crate::{Node, Visitor, expr::Expr};

///
/// ## Invoke Expression
/// `do_stuff(arg1, arg2, ...)`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvokeExpr {
    pub span: Span,
    pub target: Box<Expr>,
    pub left_paren: LParen,
    pub args: Punctuated<Expr, Comma>,
    pub right_paren: RParen,
}

impl From<InvokeExpr> for Expr {
    fn from(value: InvokeExpr) -> Self {
        Self::Invoke(value)
    }
}

impl Node for InvokeExpr {
    fn name(&self) -> &str {
        "Syntax::Expr::Invoke"
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for InvokeExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for InvokeExpr {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse<TokenParser> for InvokeExpr {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        let target = parser.parse_as::<Expr>(cursor)?;
        let left_paren = parser.parse_as::<LParen>(cursor)?;
        let args = parser.parse_as::<Punctuated<Expr, Comma>>(cursor)?;
        let right_paren = parser.parse_as::<RParen>(cursor)?;

        Ok(Self {
            span: Span::from_bounds(left_paren.span(), right_paren.span()),
            target: Box::new(target),
            left_paren,
            args,
            right_paren,
        })
    }

    fn span(&self) -> &Span {
        &self.span
    }
}
