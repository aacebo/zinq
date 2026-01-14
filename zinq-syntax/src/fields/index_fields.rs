use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::{Comma, LParen, Punctuated, RParen};

use crate::{Syntax, Visibility, spread::TypeSpread, ty::Type};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IndexField {
    pub vis: Visibility,
    pub ty: Type,
}

impl std::fmt::Display for IndexField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Peek for IndexField {
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

impl Parse for IndexField {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let vis = parser.parse::<Visibility>(cursor)?;
        let ty = parser.parse::<Type>(cursor)?;

        Ok(Self { vis, ty })
    }
}

impl Spanned for IndexField {
    fn span(&self) -> Span {
        Span::join(self.vis.span(), self.ty.span())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IndexFields {
    pub left_paren: LParen,
    pub spreads: Punctuated<TypeSpread, Comma>,
    pub fields: Punctuated<IndexField, Comma>,
    pub right_paren: RParen,
}

impl Syntax for IndexFields {
    fn name(&self) -> &str {
        "Fields::Indexed"
    }
}

impl std::ops::Deref for IndexFields {
    type Target = Punctuated<IndexField, Comma>;

    fn deref(&self) -> &Self::Target {
        &self.fields
    }
}

impl From<IndexFields> for super::Fields {
    fn from(value: IndexFields) -> Self {
        super::Fields::Indexed(value)
    }
}

impl std::fmt::Display for IndexFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Peek for IndexFields {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        if !parser.peek::<LParen>(cursor).unwrap_or(false) {
            return Ok(false);
        }

        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse for IndexFields {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let left_paren = parser.parse::<LParen>(cursor)?;
        let spreads = parser.parse::<Punctuated<TypeSpread, Comma>>(cursor)?;
        let fields = parser.parse::<Punctuated<IndexField, Comma>>(cursor)?;
        let right_paren = parser.parse::<RParen>(cursor)?;

        Ok(Self {
            left_paren,
            spreads,
            fields,
            right_paren,
        })
    }
}

impl Spanned for IndexFields {
    fn span(&self) -> Span {
        Span::join(self.left_paren.span(), self.right_paren.span())
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Index;

    use zinq_error::Result;
    use zinq_parse::Span;

    use crate::fields::IndexFields;

    #[test]
    fn should_parse_many() -> Result<()> {
        let mut cursor = Span::from_bytes(b"(string, uint, pub(super) bool)").cursor();
        let mut parser = zinq_parse::ZinqParser;
        let fields = parser.parse::<IndexFields>(&mut cursor)?;

        debug_assert_eq!(fields.len(), 3);
        debug_assert_eq!(fields.to_string(), "(string, uint, pub(super) bool)");
        debug_assert_eq!(fields.first().unwrap().value().to_string(), "string");
        debug_assert_eq!(fields.index(1).value().to_string(), "uint");
        debug_assert_eq!(
            fields.last().unwrap().value().to_string(),
            "pub(super) bool"
        );

        Ok(())
    }
}
