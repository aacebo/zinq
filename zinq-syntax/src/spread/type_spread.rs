use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::DotDot;

use crate::ty::Type;

///
/// ## Type Spread
/// ```
/// struct A {
///     pub a: u8,
/// }
///
/// struct B {
///     ..A,
///     pub b: i8,
/// }
/// ```
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeSpread {
    pub op: DotDot,
    pub ty: Type,
}

impl std::fmt::Display for TypeSpread {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Peek for TypeSpread {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser.peek::<DotDot>(cursor).unwrap_or(false))
    }
}

impl Parse for TypeSpread {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let op = parser.parse::<DotDot>(cursor)?;
        let ty = parser.parse::<Type>(cursor)?;

        Ok(Self { op, ty })
    }
}

impl Spanned for TypeSpread {
    fn span(&self) -> Span {
        Span::join(self.op.span(), self.ty.span())
    }
}
