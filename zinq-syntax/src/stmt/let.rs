use zinq_parse::{Parse, Parser, Peek, Span};
use zinq_token::{Colon, Eq, Ident, Let, SemiColon, Suffixed, TokenParser};

use crate::{Node, expr::Expr, stmt::Stmt, ty::Type};

///
/// ## Let Statement
/// `let <var>: <type> = <init>;`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LetStmt {
    pub span: Span,
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
        "Syntax::Stmt::Let"
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
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for LetStmt {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse<TokenParser> for LetStmt {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        let keyword = parser.parse_as::<Let>(cursor)?;
        let name = parser.parse_as::<Ident>(cursor)?;
        let ty = parser.parse_as::<Option<Suffixed<Colon, Type>>>(cursor)?;
        let init = parser.parse_as::<Option<Suffixed<Eq, Expr>>>(cursor)?;
        let semi = parser.parse_as::<SemiColon>(cursor)?;
        let span = Span::from_bounds(keyword.span(), semi.span());

        Ok(Self {
            span,
            keyword,
            name,
            ty,
            init,
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

    use crate::{TokenParser, stmt::LetStmt};

    #[test]
    fn should_parse_with_init() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(b"let a = b'f';").cursor();
        let ty = parser.parse_as::<LetStmt>(&mut cursor)?;

        debug_assert_eq!(ty.to_string(), "let a = b'f';");

        Ok(())
    }

    #[test]
    fn should_parse_with_type() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(b"let a: string;").cursor();
        let ty = parser.parse_as::<LetStmt>(&mut cursor)?;

        debug_assert_eq!(ty.to_string(), "let a: string;");

        Ok(())
    }

    #[test]
    fn should_parse_with_type_and_init() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(b"let a: byte = b'f';").cursor();
        let ty = parser.parse_as::<LetStmt>(&mut cursor)?;

        debug_assert_eq!(ty.to_string(), "let a: byte = b'f';");

        Ok(())
    }
}
