use zinq_parse::{Parse, Parser, Peek, Span};
use zinq_token::{Colon, Comma, Ident, LBrace, Punctuated, RBrace, zinq_parse::ZinqParser};

use crate::{Node, Visibility, Visitor, ty::Type};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedField {
    pub span: Span,
    pub vis: Visibility,
    pub name: Ident,
    pub colon: Colon,
    pub ty: Type,
}

impl std::fmt::Display for NamedField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek for NamedField {
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

impl Parse for NamedField {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let vis = parser.parse::<Visibility>(cursor)?;
        let name = parser.parse::<Ident>(cursor)?;
        let colon = parser.parse::<Colon>(cursor)?;
        let ty = parser.parse::<Type>(cursor)?;
        let span = Span::from_bounds(vis.span(), ty.span());

        Ok(Self {
            span,
            vis,
            name,
            colon,
            ty,
        })
    }

    fn span(&self) -> &zinq_parse::Span {
        &self.span
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedFields {
    pub span: Span,
    pub left_brace: LBrace,
    pub fields: Punctuated<NamedField, Comma>,
    pub right_brace: RBrace,
}

impl Node for NamedFields {
    fn name(&self) -> &str {
        "Syntax::Fields::Named"
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::ops::Deref for NamedFields {
    type Target = Punctuated<NamedField, Comma>;

    fn deref(&self) -> &Self::Target {
        &self.fields
    }
}

impl From<NamedFields> for super::Fields {
    fn from(value: NamedFields) -> Self {
        super::Fields::Named(value)
    }
}

impl std::fmt::Display for NamedFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek for NamedFields {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        if !parser.peek::<LBrace>(cursor).unwrap_or(false) {
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

impl Parse for NamedFields {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let left_brace = parser.parse::<LBrace>(cursor)?;
        let fields = parser.parse::<Punctuated<NamedField, Comma>>(cursor)?;
        let right_brace = parser.parse::<RBrace>(cursor)?;

        Ok(Self {
            span: Span::from_bounds(left_brace.span(), right_brace.span()),
            left_brace,
            fields,
            right_brace,
        })
    }

    fn span(&self) -> &zinq_parse::Span {
        &self.span
    }
}

#[cfg(test)]
mod test {
    use zinq_error::Result;
    use zinq_parse::{Parser, Span};

    use crate::{fields::NamedFields, zinq_parse::ZinqParser};

    #[test]
    fn should_parse() -> Result<()> {
        let mut cursor = Span::from_bytes(b"{ hello: string, world: u32 }").cursor();
        let mut parser = zinq_parse::ZinqParser;
        let fields = parser.parse::<NamedFields>(&mut cursor)?;

        debug_assert_eq!(fields.len(), 2);
        debug_assert_eq!(fields.to_string(), "{ hello: string, world: u32 }");
        debug_assert_eq!(fields.first().unwrap().0.to_string(), "hello: string");
        debug_assert_eq!(fields.last().unwrap().0.to_string(), "world: u32");

        Ok(())
    }

    #[test]
    fn should_parse_trailing_comma() -> Result<()> {
        let mut cursor = Span::from_bytes(b"{ hello: std::string::string, world: u32, }").cursor();
        let mut parser = zinq_parse::ZinqParser;
        let fields = parser.parse::<NamedFields>(&mut cursor)?;

        debug_assert_eq!(fields.len(), 2);
        debug_assert_eq!(
            fields.to_string(),
            "{ hello: std::string::string, world: u32, }"
        );
        debug_assert_eq!(
            fields.first().unwrap().0.to_string(),
            "hello: std::string::string"
        );
        debug_assert_eq!(fields.last().unwrap().0.to_string(), "world: u32");

        Ok(())
    }

    #[test]
    fn should_parse_formatted() -> Result<()> {
        let mut cursor = Span::from_bytes(
            b"{
            hello: string,
            pub world: u32,
        }",
        )
        .cursor();

        let mut parser = zinq_parse::ZinqParser;
        let fields = parser.parse::<NamedFields>(&mut cursor)?;

        debug_assert_eq!(fields.len(), 2);
        debug_assert_eq!(
            fields.to_string(),
            "{
            hello: string,
            pub world: u32,
        }"
        );
        debug_assert_eq!(fields.first().unwrap().0.to_string(), "hello: string");
        debug_assert_eq!(fields.last().unwrap().0.to_string(), "pub world: u32");

        Ok(())
    }
}
