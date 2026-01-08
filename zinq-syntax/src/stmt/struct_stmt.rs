use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::{Ident, SemiColon, Struct};

use crate::{Node, Visibility, fields::Fields, stmt::Stmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructStmt {
    pub vis: Visibility,
    pub keyword: Struct,
    pub name: Ident,
    pub fields: Fields,
    pub semi: Option<SemiColon>,
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
        write!(f, "{}", self.span())
    }
}

impl Peek for StructStmt {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        fork_parser.parse::<Visibility>(&mut fork)?;
        Ok(fork_parser.peek::<Struct>(&fork).unwrap_or(false))
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
        let mut semi = None;

        if fields.is_none() || fields.is_indexed() {
            semi = Some(parser.parse::<SemiColon>(cursor)?);
        }

        Ok(Self {
            vis,
            keyword,
            name,
            fields,
            semi,
        })
    }
}

impl Spanned for StructStmt {
    fn span(&self) -> Span {
        if let Some(semi) = &self.semi {
            return Span::join(self.vis.span(), semi.span());
        }

        Span::join(self.vis.span(), self.fields.span())
    }
}

#[cfg(test)]
mod test {
    use zinq_error::Result;
    use zinq_parse::Span;

    use crate::stmt::StmtParser;

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

        let stmt = parser.parse_stmt(&mut cursor)?;

        debug_assert!(stmt.is_struct());
        debug_assert!(stmt.as_struct().vis.is_priv());
        debug_assert!(stmt.as_struct().fields.is_named());
        debug_assert_eq!(stmt.as_struct().fields.len(), 2);
        debug_assert_eq!(
            stmt.to_string(),
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

        let stmt = parser.parse_stmt(&mut cursor)?;

        debug_assert!(stmt.is_struct());
        debug_assert!(stmt.as_struct().vis.is_pub());
        debug_assert!(stmt.as_struct().fields.is_named());
        debug_assert_eq!(stmt.as_struct().fields.len(), 2);
        debug_assert_eq!(
            stmt.to_string(),
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
        let mut cursor = Span::from_bytes(b"pub(mod) struct MyStruct(string, pub i32);").cursor();
        let stmt = parser.parse_stmt(&mut cursor)?;

        debug_assert!(stmt.is_struct());
        debug_assert!(stmt.as_struct().vis.is_mod());
        debug_assert!(stmt.as_struct().fields.is_indexed());
        debug_assert_eq!(stmt.as_struct().fields.len(), 2);
        debug_assert_eq!(
            stmt.to_string(),
            "pub(mod) struct MyStruct(string, pub i32);"
        );

        Ok(())
    }

    #[test]
    fn should_parse_no_fields() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"struct MyStruct;").cursor();
        let stmt = parser.parse_stmt(&mut cursor)?;

        debug_assert!(stmt.is_struct());
        debug_assert!(stmt.as_struct().fields.is_none());
        debug_assert_eq!(stmt.as_struct().fields.len(), 0);
        debug_assert_eq!(stmt.to_string(), "struct MyStruct;");

        Ok(())
    }
}
