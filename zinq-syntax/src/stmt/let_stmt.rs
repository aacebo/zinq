use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::{Colon, Eq, Ident, Let, SemiColon, Suffixed};

use crate::{Node, expr::Expr, stmt::Stmt, ty::Type};

///
/// ## Let Statement
/// `let <var>: <type> = <init>;`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LetStmt {
    pub keyword: Let,
    pub name: Ident,
    pub ty: Option<Suffixed<Colon, Type>>,
    pub init: Option<Suffixed<Eq, Expr>>,
    pub semi: SemiColon,
}

impl From<LetStmt> for Stmt {
    fn from(value: LetStmt) -> Self {
        Self::Let(value)
    }
}

impl Node for LetStmt {
    fn name(&self) -> &str {
        "Stmt::Let"
    }

    fn accept<V: crate::Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for LetStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Peek for LetStmt {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser.peek::<Let>(cursor).unwrap_or(false))
    }
}

impl Parse for LetStmt {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let keyword = parser.parse::<Let>(cursor)?;
        let name = parser.parse::<Ident>(cursor)?;
        let ty = parser.parse::<Option<Suffixed<Colon, Type>>>(cursor)?;
        let init = parser.parse::<Option<Suffixed<Eq, Expr>>>(cursor)?;
        let semi = parser.parse::<SemiColon>(cursor)?;

        Ok(Self {
            keyword,
            name,
            ty,
            init,
            semi,
        })
    }
}

impl Spanned for LetStmt {
    fn span(&self) -> Span {
        Span::join(self.keyword.span(), self.semi.span())
    }
}

#[cfg(test)]
mod test {
    use zinq_error::Result;
    use zinq_parse::Span;

    use crate::stmt::LetStmt;

    #[test]
    fn should_parse_with_init() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"let a = b'f';").cursor();
        let ty = parser.parse::<LetStmt>(&mut cursor)?;

        debug_assert_eq!(ty.to_string(), "let a = b'f';");

        Ok(())
    }

    #[test]
    fn should_parse_with_type() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"let a: string;").cursor();
        let ty = parser.parse::<LetStmt>(&mut cursor)?;

        debug_assert_eq!(ty.to_string(), "let a: string;");

        Ok(())
    }

    #[test]
    fn should_parse_with_type_and_init() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"let a: byte = b'f';").cursor();
        let ty = parser.parse::<LetStmt>(&mut cursor)?;

        debug_assert_eq!(ty.to_string(), "let a: byte = b'f';");

        Ok(())
    }
}
