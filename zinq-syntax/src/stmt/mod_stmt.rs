use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::{Ident, Mod, SemiColon};

use crate::{Node, Visibility, stmt::Stmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModStmt {
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
        write!(f, "{}", self.span())
    }
}

impl Peek for ModStmt {
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

impl Parse for ModStmt {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let vis = parser.parse::<Visibility>(cursor)?;
        let keyword = parser.parse::<Mod>(cursor)?;
        let name = parser.parse::<Ident>(cursor)?;
        let semi = parser.parse::<SemiColon>(cursor)?;

        Ok(Self {
            vis,
            keyword,
            name,
            semi,
        })
    }
}

impl Spanned for ModStmt {
    fn span(&self) -> Span {
        Span::join(self.vis.span(), self.semi.span())
    }
}

#[cfg(test)]
mod test {
    use zinq_error::Result;
    use zinq_parse::Span;

    use crate::stmt::ModStmt;

    #[test]
    fn should_parse_private() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"mod test;").cursor();

        let ty = parser.parse::<ModStmt>(&mut cursor)?;

        debug_assert!(ty.vis.is_priv());
        debug_assert_eq!(ty.to_string(), "mod test;");

        Ok(())
    }

    #[test]
    fn should_parse_public() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"pub mod test;").cursor();

        let ty = parser.parse::<ModStmt>(&mut cursor)?;

        debug_assert!(ty.vis.is_pub());
        debug_assert_eq!(ty.to_string(), "pub mod test;");

        Ok(())
    }
}
