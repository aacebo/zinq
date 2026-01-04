use zinq_parse::{Parse, Parser, Peek, Span};
use zinq_token::{Comma, LParen, Pub, Punctuated, RParen, TokenParser};

use crate::{Node, Visitor, ty::Type};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexedField {
    pub span: Span,
    pub vis: Option<Pub>,
    pub ty: Type,
}

impl std::fmt::Display for IndexedField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for IndexedField {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse<TokenParser> for IndexedField {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        let vis = parser.parse_as::<Option<Pub>>(cursor)?;
        let ty = parser.parse_as::<Type>(cursor)?;
        let mut span = ty.span().clone();

        if let Some(v) = &vis {
            span = Span::from_bounds(v.span(), ty.span())
        }

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

impl Peek<TokenParser> for IndexedFields {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        if !parser.peek_as::<LParen>(cursor).unwrap_or(false) {
            return Ok(false);
        }

        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse<TokenParser> for IndexedFields {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        let left_paren = parser.parse_as::<LParen>(cursor)?;
        let fields = parser.parse_as::<Punctuated<IndexedField, Comma>>(cursor)?;
        let right_paren = parser.parse_as::<RParen>(cursor)?;

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
    use zinq_parse::{Parser, Span};

    use crate::{TokenParser, fields::IndexedFields};

    #[test]
    fn should_parse_many() -> Result<()> {
        let mut cursor = Span::from_bytes(b"(string, uint, pub bool)").cursor();
        let mut parser = TokenParser;
        let fields = parser.parse_as::<IndexedFields>(&mut cursor)?;

        debug_assert_eq!(fields.len(), 3);
        debug_assert_eq!(fields.to_string(), "(string, uint, pub bool)");
        debug_assert_eq!(fields.first().unwrap().0.to_string(), "string");
        debug_assert_eq!(fields.index(1).0.to_string(), "uint");
        debug_assert_eq!(fields.last().unwrap().0.to_string(), "pub bool");

        Ok(())
    }
}
