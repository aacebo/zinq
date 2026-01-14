use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::{Colon, Comma, Ident, LBrace, Punctuated, RBrace};

use crate::{Syntax, Visibility, spread::TypeSpread, ty::Type};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NameField {
    pub vis: Visibility,
    pub name: Ident,
    pub colon: Colon,
    pub ty: Type,
}

impl std::fmt::Display for NameField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Peek for NameField {
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

impl Parse for NameField {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let vis = parser.parse::<Visibility>(cursor)?;
        let name = parser.parse::<Ident>(cursor)?;
        let colon = parser.parse::<Colon>(cursor)?;
        let ty = parser.parse::<Type>(cursor)?;

        Ok(Self {
            vis,
            name,
            colon,
            ty,
        })
    }
}

impl Spanned for NameField {
    fn span(&self) -> Span {
        Span::join(self.vis.span(), self.ty.span())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NameFields {
    pub left_brace: LBrace,
    pub spreads: Punctuated<TypeSpread, Comma>,
    pub fields: Punctuated<NameField, Comma>,
    pub right_brace: RBrace,
}

impl Syntax for NameFields {
    fn name(&self) -> &str {
        "Fields::Named"
    }
}

impl std::ops::Deref for NameFields {
    type Target = Punctuated<NameField, Comma>;

    fn deref(&self) -> &Self::Target {
        &self.fields
    }
}

impl From<NameFields> for super::Fields {
    fn from(value: NameFields) -> Self {
        super::Fields::Named(value)
    }
}

impl std::fmt::Display for NameFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Peek for NameFields {
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

impl Parse for NameFields {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let left_brace = parser.parse::<LBrace>(cursor)?;
        let spreads = parser.parse::<Punctuated<TypeSpread, Comma>>(cursor)?;
        let fields = parser.parse::<Punctuated<NameField, Comma>>(cursor)?;
        let right_brace = parser.parse::<RBrace>(cursor)?;

        Ok(Self {
            left_brace,
            spreads,
            fields,
            right_brace,
        })
    }
}

impl Spanned for NameFields {
    fn span(&self) -> Span {
        Span::join(self.left_brace.span(), self.right_brace.span())
    }
}

#[cfg(test)]
mod tests {
    use zinq_error::Result;
    use zinq_parse::Span;

    use crate::fields::NameFields;

    #[test]
    fn should_parse() -> Result<()> {
        let mut cursor =
            Span::from_bytes(b"{ ..std::string::String, hello: string, world: u32 }").cursor();
        let mut parser = zinq_parse::ZinqParser;
        let fields = parser.parse::<NameFields>(&mut cursor)?;

        debug_assert_eq!(fields.len(), 2);
        debug_assert_eq!(fields.spreads.len(), 1);
        debug_assert_eq!(
            fields.to_string(),
            "{ ..std::string::String, hello: string, world: u32 }"
        );
        debug_assert_eq!(fields.first().unwrap().value().to_string(), "hello: string");
        debug_assert_eq!(fields.last().unwrap().value().to_string(), "world: u32");

        Ok(())
    }

    #[test]
    fn should_parse_trailing_comma() -> Result<()> {
        let mut cursor = Span::from_bytes(b"{ hello: std::string::string, world: u32, }").cursor();
        let mut parser = zinq_parse::ZinqParser;
        let fields = parser.parse::<NameFields>(&mut cursor)?;

        debug_assert_eq!(fields.len(), 2);
        debug_assert_eq!(
            fields.to_string(),
            "{ hello: std::string::string, world: u32, }"
        );
        debug_assert_eq!(
            fields.first().unwrap().value().to_string(),
            "hello: std::string::string"
        );
        debug_assert_eq!(fields.last().unwrap().value().to_string(), "world: u32");

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
        let fields = parser.parse::<NameFields>(&mut cursor)?;

        debug_assert_eq!(fields.len(), 2);
        debug_assert_eq!(
            fields.to_string(),
            "{
            hello: string,
            pub world: u32,
        }"
        );
        debug_assert_eq!(fields.first().unwrap().value().to_string(), "hello: string");
        debug_assert_eq!(fields.last().unwrap().value().to_string(), "pub world: u32");

        Ok(())
    }
}
