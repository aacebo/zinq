use zinq_parse::{Parse, Parser, Peek, Span};
use zinq_token::{Mut, TokenParser};

use crate::{Node, ty::Type};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutType {
    pub span: Span,
    pub keyword: Mut,
    pub ty: Box<Type>,
}

impl From<MutType> for Type {
    fn from(value: MutType) -> Self {
        Type::Mut(value)
    }
}

impl Node for MutType {
    fn name(&self) -> &str {
        "Syntax::Type::Mut"
    }

    fn accept<V: crate::Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for MutType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for MutType {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse<TokenParser> for MutType {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        let keyword = parser.parse_as::<Mut>(cursor)?;
        let ty = parser.parse_as::<Box<Type>>(cursor)?;

        Ok(Self {
            span: Span::from_bounds(keyword.span(), ty.span()),
            keyword,
            ty,
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

    use crate::{TokenParser, ty::MutType};

    #[test]
    fn should_parse() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(b"mut int").cursor();
        let value = parser.parse_as::<MutType>(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "mut int");
        debug_assert_eq!(value.ty.to_string(), "int");

        Ok(())
    }
}
