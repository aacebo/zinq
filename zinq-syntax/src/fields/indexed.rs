use zinq_parse::{Parse, Peek, Span};
use zinq_token::{Comma, LParen, Punctuated, RParen};

use crate::{Node, Visibility, Visitor, ty::Type};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexedField {
    pub span: Span,
    pub vis: Visibility,
    pub ty: Type,
}

impl std::fmt::Display for IndexedField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek for IndexedField {
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

impl Parse for IndexedField {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let vis = parser.parse::<Visibility>(cursor)?;
        let ty = parser.parse::<Type>(cursor)?;
        let span = Span::from_bounds(vis.span(), ty.span());

        Ok(Self { span, vis, ty })
    }

    fn span(&self) -> &zinq_parse::Span {
        &self.span
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexedFields {
    pub span: Span,
    pub left_paren: LParen,
    pub fields: Punctuated<IndexedField, Comma>,
    pub right_paren: RParen,
}

impl Node for IndexedFields {
    fn name(&self) -> &str {
        "Syntax::Fields::Indexed"
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::ops::Deref for IndexedFields {
    type Target = Punctuated<IndexedField, Comma>;

    fn deref(&self) -> &Self::Target {
        &self.fields
    }
}

impl From<IndexedFields> for super::Fields {
    fn from(value: IndexedFields) -> Self {
        super::Fields::Indexed(value)
    }
}

impl std::fmt::Display for IndexedFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek for IndexedFields {
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

impl Parse for IndexedFields {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let left_paren = parser.parse::<LParen>(cursor)?;
        let fields = parser.parse::<Punctuated<IndexedField, Comma>>(cursor)?;
        let right_paren = parser.parse::<RParen>(cursor)?;

        Ok(Self {
            span: Span::from_bounds(left_paren.span(), right_paren.span()),
            left_paren,
            fields,
            right_paren,
        })
    }

    fn span(&self) -> &zinq_parse::Span {
        &self.span
    }
}

#[cfg(test)]
mod test {
    use std::ops::Index;

    use zinq_error::Result;
    use zinq_parse::Span;

    use crate::fields::IndexedFields;

    #[test]
    fn should_parse_many() -> Result<()> {
        let mut cursor = Span::from_bytes(b"(string, uint, pub(super) bool)").cursor();
        let mut parser = zinq_parse::ZinqParser;
        let fields = parser.parse::<IndexedFields>(&mut cursor)?;

        debug_assert_eq!(fields.len(), 3);
        debug_assert_eq!(fields.to_string(), "(string, uint, pub(super) bool)");
        debug_assert_eq!(fields.first().unwrap().0.to_string(), "string");
        debug_assert_eq!(fields.index(1).0.to_string(), "uint");
        debug_assert_eq!(fields.last().unwrap().0.to_string(), "pub(super) bool");

        Ok(())
    }
}
