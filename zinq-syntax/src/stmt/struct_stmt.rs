use zinq_parse::{Parse, Peek, Span};
use zinq_token::{Ident, Struct};

use crate::{Node, Visibility, fields::Fields, stmt::Stmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructStmt {
    pub span: Span,
    pub vis: Visibility,
    pub keyword: Struct,
    pub name: Ident,
    pub fields: Fields,
}

impl From<StructStmt> for Stmt {
    fn from(value: StructStmt) -> Self {
        Self::Struct(value)
    }
}

impl Node for StructStmt {
    fn name(&self) -> &str {
        "Syntax::Stmt::Struct"
    }

    fn accept<V: crate::Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for StructStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek for StructStmt {
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

impl Parse for StructStmt {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let vis = parser.parse::<Visibility>(cursor)?;
        let keyword = parser.parse::<Struct>(cursor)?;
        let name = parser.parse::<Ident>(cursor)?;
        let fields = parser.parse::<Fields>(cursor)?;
        let span = Span::from_bounds(vis.span(), fields.span());

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
    use zinq_parse::Span;

    use crate::stmt::StructStmt;

    #[test]
    fn should_parse_private() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(
            b"struct MyStruct {
            a: i8,
            b: string,
        }",
        )
        .cursor();

        let ty = parser.parse::<StructStmt>(&mut cursor)?;

        debug_assert!(ty.vis.is_priv());
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
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(
            b"pub struct MyStruct {
            pub a: i8,
            b: string,
        }",
        )
        .cursor();

        let ty = parser.parse::<StructStmt>(&mut cursor)?;

        debug_assert!(ty.vis.is_pub());
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
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"pub(mod) struct MyStruct(string, pub i32)").cursor();
        let ty = parser.parse::<StructStmt>(&mut cursor)?;

        debug_assert!(ty.vis.is_mod());
        debug_assert_eq!(ty.fields.len(), 2);
        debug_assert_eq!(ty.to_string(), "pub(mod) struct MyStruct(string, pub i32)");

        Ok(())
    }
}
