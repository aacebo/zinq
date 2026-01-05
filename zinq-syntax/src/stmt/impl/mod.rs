mod syntax;

pub use syntax::*;
use zinq_parse::{Parse, Parser, Peek, Span};
use zinq_token::{Impl, LBrace, RBrace, TokenParser};

use crate::{Node, stmt::Stmt, ty::Type};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImplStmt {
    pub span: Span,
    pub keyword: Impl,
    pub for_ty: Type,
    pub left_brace: LBrace,
    pub stmts: Vec<ImplSyntax>,
    pub right_brace: RBrace,
}

impl From<ImplStmt> for Stmt {
    fn from(value: ImplStmt) -> Self {
        Self::Impl(value)
    }
}

impl Node for ImplStmt {
    fn name(&self) -> &str {
        "Syntax::Stmt::Impl"
    }

    fn accept<V: crate::Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for ImplStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for ImplStmt {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse<TokenParser> for ImplStmt {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        let keyword = parser.parse_as::<Impl>(cursor)?;
        let for_ty = parser.parse_as::<Type>(cursor)?;
        let left_brace = parser.parse_as::<LBrace>(cursor)?;
        let mut stmts = vec![];

        while parser.peek_as::<ImplSyntax>(cursor).unwrap_or(false) {
            stmts.push(parser.parse_as::<ImplSyntax>(cursor)?);
        }

        let right_brace = parser.parse_as::<RBrace>(cursor)?;

        Ok(Self {
            span: Span::from_bounds(keyword.span(), right_brace.span()),
            keyword,
            for_ty,
            left_brace,
            stmts,
            right_brace,
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

    use crate::{TokenParser, stmt::ImplStmt};

    #[test]
    fn should_parse() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(
            b"impl Message {
                pub fn new(text: string) -> Self {
                    Self { text }
                }

                pub fn to_string(self) -> string { self.text }
            }",
        )
        .cursor();

        let ty = parser.parse_as::<ImplStmt>(&mut cursor)?;

        debug_assert_eq!(
            ty.to_string(),
            "impl Message {
                pub fn new(text: string) -> Self {
                    Self { text }
                }

                pub fn to_string(self) -> string { self.text }
            }"
        );

        Ok(())
    }
}
