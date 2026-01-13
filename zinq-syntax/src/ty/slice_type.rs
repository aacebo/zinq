use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::{LBracket, RBracket};

use crate::{Node, ty::Type};

///
/// ## Slice Type
/// `[T]`
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SliceType {
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
        "Type::Slice"
    }
}

impl std::fmt::Display for SliceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Peek for SliceType {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser.peek::<LBracket>(cursor).unwrap_or(false))
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
            left_bracket,
            item_ty,
            right_bracket,
        })
    }
}

impl Spanned for SliceType {
    fn span(&self) -> Span {
        Span::join(self.left_bracket.span(), self.right_bracket.span())
    }
}

#[cfg(test)]
mod tests {
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
