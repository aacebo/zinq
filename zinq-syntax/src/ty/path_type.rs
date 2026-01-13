use zinq_parse::{Parse, Peek, Span, Spanned};

use crate::{Node, Path, ty::Type};

///
/// ## Path Type
/// `T`
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathType {
    pub inner: Path,
}

impl From<PathType> for Type {
    fn from(value: PathType) -> Self {
        Self::Path(value)
    }
}

impl Node for PathType {
    fn name(&self) -> &str {
        "Type::Path"
    }

    fn accept<V: crate::Visitor>(&self, visitor: &mut V) {
        visitor.visit_path_type(self);
    }
}

impl std::fmt::Display for PathType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Peek for PathType {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser.peek::<Path>(cursor).unwrap_or(false))
    }
}

impl Parse for PathType {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let inner = parser.parse::<Path>(cursor)?;
        Ok(Self { inner })
    }
}

impl Spanned for PathType {
    fn span(&self) -> Span {
        self.inner.span()
    }
}

#[cfg(test)]
mod tests {
    use zinq_error::Result;
    use zinq_parse::Span;

    use crate::ty::PathType;

    #[test]
    fn should_parse() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"int").cursor();
        let value = parser.parse::<PathType>(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "int");

        Ok(())
    }
}
