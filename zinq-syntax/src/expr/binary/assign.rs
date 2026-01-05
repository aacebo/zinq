use zinq_parse::{Parse, Peek, Span};
use zinq_token::Eq;

use crate::{
    Node, Visitor,
    expr::{BinaryExpr, Expr},
};

///
/// ## Assign Expression
/// `message = (...)`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssignExpr {
    pub span: Span,
    pub left: Box<Expr>,
    pub eq: Eq,
    pub right: Box<Expr>,
}

impl From<AssignExpr> for BinaryExpr {
    fn from(value: AssignExpr) -> Self {
        Self::Assign(value)
    }
}

impl Node for AssignExpr {
    fn name(&self) -> &str {
        "Syntax::Expr::Binary::Assign"
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

impl Peek for AssignExpr {
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

impl Parse for AssignExpr {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let left = parser.parse::<Box<Expr>>(cursor)?;
        let eq = parser.parse::<Eq>(cursor)?;
        let right = parser.parse::<Box<Expr>>(cursor)?;

        Ok(Self {
            span: Span::from_bounds(left.span(), right.span()),
            left,
            eq,
            right,
        })
    }

    fn span(&self) -> &Span {
        &self.span
    }
}

#[cfg(test)]
mod test {
    use zinq_error::Result;
    use zinq_parse::Span;

    use crate::expr::AssignExpr;

    #[test]
    fn should_parse() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"a = b'h'").cursor();
        let value = parser.parse::<AssignExpr>(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "a = b'h'");
        debug_assert_eq!(value.left.to_string(), "a");
        debug_assert_eq!(value.right.to_string(), "b'h'");

        Ok(())
    }
}
