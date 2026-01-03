use zinq_parse::{Parse, Parser, Peek, Span};
use zinq_token::{Eq, TokenParser};

use crate::{
    Node, Visitor,
    expr::{Expr, GetFieldExpr},
};

///
/// ## Set Field Expression
/// `my_var.test = 1 + 5`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetFieldExpr {
    pub span: Span,
    pub field: GetFieldExpr,
    pub eq: Eq,
    pub value: Box<Expr>,
}

impl From<SetFieldExpr> for Expr {
    fn from(value: SetFieldExpr) -> Self {
        Self::SetField(value)
    }
}

impl Node for SetFieldExpr {
    fn name(&self) -> &str {
        "Syntax::Expr::Set::Field"
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for SetFieldExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for SetFieldExpr {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse<TokenParser> for SetFieldExpr {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        let field = parser.parse_as::<GetFieldExpr>(cursor)?;
        let eq = parser.parse_as::<Eq>(cursor)?;
        let value = parser.parse_as::<Box<Expr>>(cursor)?;

        Ok(Self {
            span: Span::from_bounds(field.span(), value.span()),
            field,
            eq,
            value,
        })
    }

    fn span(&self) -> &Span {
        &self.span
    }
}

#[cfg(test)]
mod test {
    use zinq_error::Result;
    use zinq_parse::{Parser, Span};

    use crate::{TokenParser, expr::SetFieldExpr};

    #[test]
    fn should_parse() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(b"a.message = b'h'").cursor();
        let value = parser.parse_as::<SetFieldExpr>(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "a.message = b'h'");
        debug_assert_eq!(value.field.to_string(), "a.message");

        debug_assert!(value.value.is_literal());
        debug_assert_eq!(value.value.to_string(), "b'h'");

        Ok(())
    }
}
