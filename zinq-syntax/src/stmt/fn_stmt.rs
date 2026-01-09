use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::{Comma, Fn, Ident, LParen, Punctuated, RArrow, RParen, Suffixed};

use crate::{
    Generics, Node, Visibility,
    param::FnParam,
    stmt::{BlockStmt, Stmt},
    ty::Type,
};

///
/// ## Fn Statement
/// `fn <name>(<arg1>, <arg2>, ...) -> <return_type> { ... }`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FnStmt {
    pub vis: Visibility,
    pub keyword: Fn,
    pub name: Ident,
    pub generics: Option<Generics>,
    pub left_paren: LParen,
    pub params: Punctuated<FnParam, Comma>,
    pub right_paren: RParen,
    pub return_ty: Option<Suffixed<RArrow, Type>>,
    pub block: BlockStmt,
}

impl From<FnStmt> for Stmt {
    fn from(value: FnStmt) -> Self {
        Self::Fn(value)
    }
}

impl Node for FnStmt {
    fn name(&self) -> &str {
        "Syntax::Stmt::Fn"
    }

    fn accept<V: crate::Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for FnStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Peek for FnStmt {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        fork_parser.parse::<Visibility>(&mut fork)?;
        Ok(fork_parser.peek::<Fn>(&fork).unwrap_or(false))
    }
}

impl Parse for FnStmt {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let vis = parser.parse::<Visibility>(cursor)?;
        let keyword = parser.parse::<Fn>(cursor)?;
        let name = parser.parse::<Ident>(cursor)?;
        let generics = parser.parse::<Option<Generics>>(cursor)?;
        let left_paren = parser.parse::<LParen>(cursor)?;
        let params = parser.parse::<Punctuated<FnParam, Comma>>(cursor)?;
        let right_paren = parser.parse::<RParen>(cursor)?;
        let return_ty = parser.parse::<Option<Suffixed<RArrow, Type>>>(cursor)?;
        let block = parser.parse::<BlockStmt>(cursor)?;

        Ok(Self {
            vis,
            keyword,
            name,
            generics,
            left_paren,
            params,
            right_paren,
            return_ty,
            block,
        })
    }
}

impl Spanned for FnStmt {
    fn span(&self) -> Span {
        Span::join(self.vis.span(), self.block.span())
    }
}

#[cfg(test)]
mod test {
    use zinq_error::Result;
    use zinq_parse::Span;

    use crate::stmt::FnStmt;

    #[test]
    fn should_parse() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"fn stuff() { }").cursor();
        let ty = parser.parse::<FnStmt>(&mut cursor)?;

        debug_assert_eq!(ty.to_string(), "fn stuff() { }");

        Ok(())
    }

    #[test]
    fn should_parse_with_visibility() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"pub(super) fn stuff() { }").cursor();
        let ty = parser.parse::<FnStmt>(&mut cursor)?;

        debug_assert_eq!(ty.to_string(), "pub(super) fn stuff() { }");

        Ok(())
    }

    #[test]
    fn should_parse_with_return_type() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"fn stuff() -> models::User { }").cursor();
        let ty = parser.parse::<FnStmt>(&mut cursor)?;

        debug_assert_eq!(ty.to_string(), "fn stuff() -> models::User { }");

        Ok(())
    }

    #[test]
    fn should_parse_with_params() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"fn stuff(a: string, b: u32) { }").cursor();
        let ty = parser.parse::<FnStmt>(&mut cursor)?;

        debug_assert_eq!(ty.to_string(), "fn stuff(a: string, b: u32) { }");
        debug_assert_eq!(ty.params.len(), 2);

        Ok(())
    }
}
