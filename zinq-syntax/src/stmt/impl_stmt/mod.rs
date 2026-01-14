mod syntax;

pub use syntax::*;
use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::{Impl, LBrace, RBrace};

use crate::{Generics, Syntax, stmt::Stmt, ty::Type};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImplStmt {
    pub keyword: Impl,
    pub generics: Option<Generics>,
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

impl Syntax for ImplStmt {
    fn name(&self) -> &str {
        "Stmt::Impl"
    }

    fn accept<V: crate::Visitor>(&self, visitor: &mut V) {
        visitor.visit_impl_stmt(self);
        self.for_ty.accept(visitor);
    }
}

impl std::fmt::Display for ImplStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Peek for ImplStmt {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser.peek::<Impl>(cursor).unwrap_or(false))
    }
}

impl Parse for ImplStmt {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let keyword = parser.parse::<Impl>(cursor)?;
        let generics = parser.parse::<Option<Generics>>(cursor)?;
        let for_ty = parser.parse::<Type>(cursor)?;
        let left_brace = parser.parse::<LBrace>(cursor)?;
        let mut stmts = vec![];

        while parser.peek::<ImplSyntax>(cursor).unwrap_or(false) {
            stmts.push(parser.parse::<ImplSyntax>(cursor)?);
        }

        let right_brace = parser.parse::<RBrace>(cursor)?;

        Ok(Self {
            keyword,
            generics,
            for_ty,
            left_brace,
            stmts,
            right_brace,
        })
    }
}

impl Spanned for ImplStmt {
    fn span(&self) -> Span {
        Span::join(self.keyword.span(), self.right_brace.span())
    }
}
#[cfg(test)]
mod tests {
    use zinq_error::Result;
    use zinq_parse::Span;

    use crate::stmt::ImplStmt;

    #[test]
    fn should_parse() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(
            b"impl Message {
                pub fn new(text: string) -> Self {
                    Self { text }
                }

                pub fn to_string(self) -> string { self.text }
            }",
        )
        .cursor();

        let ty = parser.parse::<ImplStmt>(&mut cursor)?;

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
