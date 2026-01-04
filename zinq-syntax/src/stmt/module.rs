use zinq_parse::{Parse, Parser, Peek, Span};
use zinq_token::{Ident, Mod, SemiColon, TokenParser};

use crate::{Node, Visibility, stmt::Stmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModStmt {
    pub span: Span,
    pub vis: Visibility,
    pub keyword: Mod,
    pub name: Ident,
    pub semi: SemiColon,
}

impl From<ModStmt> for Stmt {
    fn from(value: ModStmt) -> Self {
        Self::Mod(value)
    }
}

impl Node for ModStmt {
    fn name(&self) -> &str {
        "Syntax::Stmt::Mod"
    }

    fn accept<V: crate::Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for ModStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for ModStmt {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse<TokenParser> for ModStmt {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        let vis = parser.parse_as::<Visibility>(cursor)?;
        let keyword = parser.parse_as::<Mod>(cursor)?;
        let name = parser.parse_as::<Ident>(cursor)?;
        let semi = parser.parse_as::<SemiColon>(cursor)?;
        let span = Span::from_bounds(vis.span(), semi.span());

        Ok(Self {
            span,
            vis,
            keyword,
            name,
            semi,
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

    use crate::{TokenParser, stmt::ModStmt};

    #[test]
    fn should_parse_private() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(b"mod test;").cursor();

        let ty = parser.parse_as::<ModStmt>(&mut cursor)?;

        debug_assert!(ty.vis.is_priv());
        debug_assert_eq!(ty.to_string(), "mod test;");

        Ok(())
    }

    #[test]
    fn should_parse_public() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(b"pub mod test;").cursor();

        let ty = parser.parse_as::<ModStmt>(&mut cursor)?;

        debug_assert!(ty.vis.is_pub());
        debug_assert_eq!(ty.to_string(), "pub mod test;");

        Ok(())
    }
}
