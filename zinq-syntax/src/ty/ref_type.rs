use zinq_parse::{Parse, Parser, Peek, Span};
use zinq_token::{And, Ident, TokenParser};

use crate::{Node, ty::Type};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefType {
    pub span: Span,
    pub and: And,
    pub expr: Ident,
}

impl From<RefType> for Type {
    fn from(value: RefType) -> Self {
        Type::Ref(value)
    }
}

impl Node for RefType {
    fn name(&self) -> &str {
        "Syntax::Type::Ref"
    }

    fn accept<V: crate::Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for RefType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for RefType {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse<TokenParser> for RefType {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        let and = parser.parse_as::<And>(cursor)?;
        let expr = parser.parse_as::<Ident>(cursor)?;

        Ok(Self {
            span: Span::from_bounds(and.span(), expr.span()),
            and,
            expr,
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

    use crate::{TokenParser, ty::RefType};

    #[test]
    fn should_parse() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(b"&int").cursor();
        let value = parser.parse_as::<RefType>(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "&int");
        debug_assert_eq!(value.expr.to_string(), "int");

        Ok(())
    }
}
