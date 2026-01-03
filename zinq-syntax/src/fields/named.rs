use zinq_parse::{Parse, Parser, Peek, Span};
use zinq_token::{Colon, Comma, Ident, LBrace, Pub, Punctuated, RBrace, TokenParser};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedField {
    pub span: Span,
    pub vis: Option<Pub>,
    pub name: Ident,
    pub colon: Colon,
    pub ty: Ident,
}

impl std::fmt::Display for NamedField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for NamedField {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse<TokenParser> for NamedField {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        let vis = parser.parse_as::<Option<Pub>>(cursor)?;
        let name = parser.parse_as::<Ident>(cursor)?;
        let colon = parser.parse_as::<Colon>(cursor)?;
        let ty = parser.parse_as::<Ident>(cursor)?;
        let mut span = Span::from_bounds(name.span(), ty.span());

        if let Some(v) = &vis {
            span = Span::from_bounds(v.span(), ty.span())
        }

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

impl Peek<TokenParser> for NamedFields {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        if !parser.peek_as::<LBrace>(cursor).unwrap_or(false) {
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

impl Parse<TokenParser> for NamedFields {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        let left_brace = parser.parse_as::<LBrace>(cursor)?;
        let fields = parser.parse_as::<Punctuated<NamedField, Comma>>(cursor)?;
        let right_brace = parser.parse_as::<RBrace>(cursor)?;

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

    use crate::{TokenParser, fields::NamedFields};

    #[test]
    fn should_parse() -> Result<()> {
        let mut cursor = Span::from_bytes(b"{ hello: string, world: u32 }").cursor();
        let mut parser = TokenParser;
        let fields = parser.parse_as::<NamedFields>(&mut cursor)?;

        debug_assert_eq!(fields.len(), 2);
        debug_assert_eq!(fields.to_string(), "{ hello: string, world: u32 }");
        debug_assert_eq!(fields.first().unwrap().0.to_string(), "hello: string");
        debug_assert_eq!(fields.last().unwrap().0.to_string(), "world: u32");

        Ok(())
    }

    #[test]
    fn should_parse_trailing_comma() -> Result<()> {
        let mut cursor = Span::from_bytes(b"{ hello: string, world: u32, }").cursor();
        let mut parser = TokenParser;
        let fields = parser.parse_as::<NamedFields>(&mut cursor)?;

        debug_assert_eq!(fields.len(), 2);
        debug_assert_eq!(fields.to_string(), "{ hello: string, world: u32, }");
        debug_assert_eq!(fields.first().unwrap().0.to_string(), "hello: string");
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

        let mut parser = TokenParser;
        let fields = parser.parse_as::<NamedFields>(&mut cursor)?;

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
