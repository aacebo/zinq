use zinq_parse::{Parse, Parser, Peek, Span};

use crate::{
    Ident, Pub, Struct, TokenParser,
    syntax::{fields::Fields, ty},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructType {
    pub span: Span,
    pub vis: Option<Pub>,
    pub keyword: Struct,
    pub name: Ident,
    pub fields: Fields,
}

impl From<StructType> for ty::Type {
    fn from(value: StructType) -> Self {
        ty::Type::Struct(value)
    }
}

impl std::fmt::Display for StructType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for StructType {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        if fork_parser.peek_as::<Pub>(&mut fork).unwrap_or(false) {
            fork_parser.parse_as::<Pub>(&mut fork)?;
        }

        if fork_parser.peek_as::<Struct>(&mut fork).unwrap_or(false) {
            fork_parser.parse_as::<Struct>(&mut fork)?;
        }

        Ok(true)
    }
}

impl Parse<TokenParser> for StructType {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        let vis = parser.parse_as::<Option<Pub>>(cursor)?;
        let keyword = parser.parse_as::<Struct>(cursor)?;
        let name = parser.parse_as::<Ident>(cursor)?;
        let fields = parser.parse_as::<Fields>(cursor)?;
        let mut span = Span::from_bounds(keyword.span(), fields.span());

        if let Some(v) = &vis {
            span = Span::from_bounds(v.span(), fields.span());
        }

        Ok(Self {
            span,
            vis,
            keyword,
            name,
            fields,
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

    use crate::{TokenParser, syntax::ty};

    #[test]
    fn should_parse_private() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(
            b"struct MyStruct {
            a: i8,
            b: string,
        }",
        )
        .cursor();

        let ty = parser.parse_as::<ty::StructType>(&mut cursor)?;

        debug_assert!(ty.vis.is_none());
        debug_assert_eq!(ty.fields.len(), 2);
        debug_assert_eq!(
            ty.to_string(),
            "struct MyStruct {
            a: i8,
            b: string,
        }"
        );

        Ok(())
    }

    #[test]
    fn should_parse_public() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(
            b"pub struct MyStruct {
            pub a: i8,
            b: string,
        }",
        )
        .cursor();

        let ty = parser.parse_as::<ty::StructType>(&mut cursor)?;

        debug_assert!(ty.vis.is_some());
        debug_assert_eq!(ty.fields.len(), 2);
        debug_assert_eq!(
            ty.to_string(),
            "pub struct MyStruct {
            pub a: i8,
            b: string,
        }"
        );

        Ok(())
    }

    #[test]
    fn should_parse_indexed() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(b"pub struct MyStruct(string, pub i32)").cursor();
        let ty = parser.parse_as::<ty::StructType>(&mut cursor)?;

        debug_assert!(ty.vis.is_some());
        debug_assert_eq!(ty.fields.len(), 2);
        debug_assert_eq!(ty.to_string(), "pub struct MyStruct(string, pub i32)");

        Ok(())
    }
}
