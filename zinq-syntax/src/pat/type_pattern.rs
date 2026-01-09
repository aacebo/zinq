use zinq_parse::{Parse, Peek, Spanned};

use crate::{pat::Pattern, ty::Type};

///
/// ## Type Pattern
/// - `std::string::String => ..`
/// - `u8 => ..`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypePattern {
    pub ty: Type,
}

impl std::ops::Deref for TypePattern {
    type Target = Type;

    fn deref(&self) -> &Self::Target {
        &self.ty
    }
}

impl From<Type> for TypePattern {
    fn from(value: Type) -> Self {
        Self { ty: value }
    }
}

impl From<TypePattern> for Pattern {
    fn from(value: TypePattern) -> Self {
        Self::Type(value)
    }
}

impl std::fmt::Display for TypePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Spanned for TypePattern {
    fn span(&self) -> zinq_parse::Span {
        self.ty.span()
    }
}

impl Peek for TypePattern {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser.peek::<Type>(cursor).unwrap_or(false))
    }
}

impl Parse for TypePattern {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let ty = parser.parse::<Type>(cursor)?;
        Ok(Self { ty })
    }
}
