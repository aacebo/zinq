use zinq_parse::{Parse, Parser, Peek, Span};
use zinq_token::{Comma, Fn, Ident, LParen, Punctuated, RArrow, RParen, Suffixed, TokenParser};

use crate::{
    Node, Visibility,
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
    pub span: Span,
    pub vis: Visibility,
    pub keyword: Fn,
    pub name: Ident,
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
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for FnStmt {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse<TokenParser> for FnStmt {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        let vis = parser.parse_as::<Visibility>(cursor)?;
        let keyword = parser.parse_as::<Fn>(cursor)?;
        let name = parser.parse_as::<Ident>(cursor)?;
        let left_paren = parser.parse_as::<LParen>(cursor)?;
        let params = parser.parse_as::<Punctuated<FnParam, Comma>>(cursor)?;
        let right_paren = parser.parse_as::<RParen>(cursor)?;
        let return_ty = parser.parse_as::<Option<Suffixed<RArrow, Type>>>(cursor)?;
        let block = parser.parse_as::<BlockStmt>(cursor)?;
        let span = Span::from_bounds(vis.span(), block.span());

        Ok(Self {
            span,
            vis,
            keyword,
            name,
            left_paren,
            params,
            right_paren,
            return_ty,
            block,
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

    use crate::{TokenParser, stmt::FnStmt};

    #[test]
    fn should_parse() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(b"fn stuff() { }").cursor();
        let ty = parser.parse_as::<FnStmt>(&mut cursor)?;

        debug_assert_eq!(ty.to_string(), "fn stuff() { }");

        Ok(())
    }

    #[test]
    fn should_parse_with_visibility() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(b"pub(super) fn stuff() { }").cursor();
        let ty = parser.parse_as::<FnStmt>(&mut cursor)?;

        debug_assert_eq!(ty.to_string(), "pub(super) fn stuff() { }");

        Ok(())
    }

    #[test]
    fn should_parse_with_return_type() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(b"fn stuff() -> models::User { }").cursor();
        let ty = parser.parse_as::<FnStmt>(&mut cursor)?;

        debug_assert_eq!(ty.to_string(), "fn stuff() -> models::User { }");

        Ok(())
    }

    #[test]
    fn should_parse_with_params() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(b"fn stuff(a: string, b: u32) { }").cursor();
        let ty = parser.parse_as::<FnStmt>(&mut cursor)?;

        debug_assert_eq!(ty.to_string(), "fn stuff(a: string, b: u32) { }");
        debug_assert_eq!(ty.params.len(), 2);

        Ok(())
    }
}
