use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::And;

use crate::{Node, ty::Type};

///
/// ## Reference Type
/// `&T`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefType {
    pub and: And,
    pub to: Box<Type>,
}

impl From<RefType> for Type {
    fn from(value: RefType) -> Self {
        Self::Ref(value)
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
        write!(f, "{}", self.span())
    }
}

impl Peek for RefType {
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

impl Parse for RefType {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let and = parser.parse::<And>(cursor)?;
        let to = parser.parse::<Box<Type>>(cursor)?;

        Ok(Self { and, to })
    }
}

impl Spanned for RefType {
    fn span(&self) -> Span {
        Span::join(self.and.span(), self.to.span())
    }
}

#[cfg(test)]
mod test {
    use zinq_error::Result;
    use zinq_parse::Span;

    use crate::ty::RefType;

    #[test]
    fn should_parse() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"&int").cursor();
        let value = parser.parse::<RefType>(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "&int");
        debug_assert_eq!(value.to.to_string(), "int");

        Ok(())
    }

    #[test]
    fn should_parse_mut() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"&mut int").cursor();
        let value = parser.parse::<RefType>(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "&mut int");
        debug_assert_eq!(value.to.to_string(), "mut int");

        Ok(())
    }
}
