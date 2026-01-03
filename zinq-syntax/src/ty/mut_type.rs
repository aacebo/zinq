use zinq_parse::{Parse, Parser, Peek, Span};
use zinq_token::{Ident, Mut, TokenParser};

use crate::ty::Type;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutType {
    pub span: Span,
    pub keyword: Mut,
    pub ty: Ident,
}

impl From<MutType> for Type {
    fn from(value: MutType) -> Self {
        Type::Mut(value)
    }
}

impl std::fmt::Display for MutType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for MutType {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        Ok(parser.peek_as::<Mut>(cursor).unwrap_or(false))
    }
}

impl Parse<TokenParser> for MutType {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        let keyword = parser.parse_as::<Mut>(cursor)?;
        let ty = parser.parse_as::<Ident>(cursor)?;

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
