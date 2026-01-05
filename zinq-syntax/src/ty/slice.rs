use zinq_parse::{Parse, Peek, Span};
use zinq_token::{LBracket, RBracket};

use crate::{Node, ty::Type};

///
/// ## Slice Type
/// `[T]`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SliceType {
    pub span: Span,
    pub left_bracket: LBracket,
    pub item_ty: Box<Type>,
    pub right_bracket: RBracket,
}

impl From<SliceType> for Type {
    fn from(value: SliceType) -> Self {
        Self::Slice(value)
    }
}

impl Node for SliceType {
    fn name(&self) -> &str {
        "Syntax::Type::Slice"
    }

    fn accept<V: crate::Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for SliceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek for SliceType {
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

impl Parse for SliceType {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let left_bracket = parser.parse::<LBracket>(cursor)?;
        let item_ty = parser.parse::<Box<Type>>(cursor)?;
        let right_bracket = parser.parse::<RBracket>(cursor)?;

        Ok(Self {
            span: Span::from_bounds(left_bracket.span(), right_bracket.span()),
            left_bracket,
            item_ty,
            right_bracket,
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

    use crate::ty::SliceType;

    #[test]
    fn should_parse() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"[u32]").cursor();
        let value = parser.parse::<SliceType>(&mut cursor)?;

        debug_assert_eq!(value.to_string(), "[u32]");
        debug_assert_eq!(value.item_ty.to_string(), "u32");

        Ok(())
    }
}
