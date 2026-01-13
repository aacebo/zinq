use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::{Comma, Enum, Ident, LBrace, Punctuated, RBrace};

use crate::{Generics, Node, Variant, Visibility, meta::Meta, stmt::Stmt};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnumStmt {
    pub meta: Option<Meta>,
    pub vis: Visibility,
    pub keyword: Enum,
    pub name: Ident,
    pub generics: Option<Generics>,
    pub left_brace: LBrace,
    pub variants: Punctuated<Variant, Comma>,
    pub right_brace: RBrace,
}

impl From<EnumStmt> for Stmt {
    fn from(value: EnumStmt) -> Self {
        Self::Enum(value)
    }
}

impl std::fmt::Display for EnumStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Node for EnumStmt {
    fn name(&self) -> &str {
        "Stmt::Enum"
    }
}

impl Peek for EnumStmt {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        fork_parser.parse::<Option<Meta>>(&mut fork)?;
        fork_parser.parse::<Visibility>(&mut fork)?;
        Ok(fork_parser.peek::<Enum>(&fork).unwrap_or(false))
    }
}

impl Parse for EnumStmt {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let meta = parser.parse::<Option<Meta>>(cursor)?;
        let vis = parser.parse::<Visibility>(cursor)?;
        let keyword = parser.parse::<Enum>(cursor)?;
        let name = parser.parse::<Ident>(cursor)?;
        let generics = parser.parse::<Option<Generics>>(cursor)?;
        let left_brace = parser.parse::<LBrace>(cursor)?;
        let variants = parser.parse::<Punctuated<Variant, Comma>>(cursor)?;
        let right_brace = parser.parse::<RBrace>(cursor)?;

        Ok(Self {
            meta,
            vis,
            keyword,
            name,
            generics,
            left_brace,
            variants,
            right_brace,
        })
    }
}

impl Spanned for EnumStmt {
    fn span(&self) -> zinq_parse::Span {
        if let Some(meta) = &self.meta {
            return Span::join(meta.span(), self.right_brace.span());
        }

        Span::join(self.vis.span(), self.right_brace.span())
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Index;

    use zinq_error::Result;
    use zinq_parse::Span;

    use crate::stmt::StmtParser;

    #[test]
    fn should_parse() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(
            b"enum Status {
            Away,
            Online(DateTime),
            Offline {
                last_online_at: DateTime,
            },
        }",
        )
        .cursor();

        let stmt = parser.parse_stmt(&mut cursor)?;

        debug_assert!(stmt.is_enum());
        debug_assert_eq!(stmt.as_enum().variants.len(), 3);
        debug_assert!(stmt.as_enum().variants.index(0).value().fields.is_none());
        debug_assert!(stmt.as_enum().variants.index(1).value().fields.is_indexed());
        debug_assert!(stmt.as_enum().variants.index(2).value().fields.is_named());
        debug_assert_eq!(
            stmt.to_string(),
            "enum Status {
            Away,
            Online(DateTime),
            Offline {
                last_online_at: DateTime,
            },
        }"
        );

        Ok(())
    }

    #[test]
    fn should_parse_with_meta() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(
            b"#[Debug]
            enum Status {
            Away,
            Online(DateTime),
            Offline {
                last_online_at: DateTime,
            },
        }",
        )
        .cursor();

        let stmt = parser.parse_stmt(&mut cursor)?;

        debug_assert!(stmt.is_enum());
        debug_assert_eq!(stmt.as_enum().variants.len(), 3);
        debug_assert!(stmt.as_enum().variants.index(0).value().fields.is_none());
        debug_assert!(stmt.as_enum().variants.index(1).value().fields.is_indexed());
        debug_assert!(stmt.as_enum().variants.index(2).value().fields.is_named());
        debug_assert_eq!(
            stmt.to_string(),
            "#[Debug]
            enum Status {
            Away,
            Online(DateTime),
            Offline {
                last_online_at: DateTime,
            },
        }"
        );

        Ok(())
    }
}
