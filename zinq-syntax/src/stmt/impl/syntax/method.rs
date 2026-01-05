use zinq_parse::{Parse, Parser, Peek, Span};
use zinq_token::{Comma, Fn, Ident, LParen, Punctuated, RArrow, RParen, Suffixed, TokenParser};

use crate::{
    Node, Visibility,
    param::{FnParam, SelfParam},
    stmt::{BlockStmt, ImplSyntax},
    ty::Type,
};

///
/// ## Impl Method Statement
/// `fn <name>(<self_param>, <arg1>, <arg2>, ...) -> <return_type> { ... }`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImplMethod {
    pub span: Span,
    pub vis: Visibility,
    pub keyword: Fn,
    pub name: Ident,
    pub left_paren: LParen,
    pub self_param: Option<SelfParam>,
    pub params: Punctuated<FnParam, Comma>,
    pub right_paren: RParen,
    pub return_ty: Option<Suffixed<RArrow, Type>>,
    pub block: BlockStmt,
}

impl From<ImplMethod> for ImplSyntax {
    fn from(value: ImplMethod) -> Self {
        Self::Method(value)
    }
}

impl Node for ImplMethod {
    fn name(&self) -> &str {
        "Syntax::Stmt::Impl::Method"
    }

    fn accept<V: crate::Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for ImplMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for ImplMethod {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse<TokenParser> for ImplMethod {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        let vis = parser.parse_as::<Visibility>(cursor)?;
        let keyword = parser.parse_as::<Fn>(cursor)?;
        let name = parser.parse_as::<Ident>(cursor)?;
        let left_paren = parser.parse_as::<LParen>(cursor)?;
        let self_param = parser.parse_as::<Option<SelfParam>>(cursor)?;

        if self_param.is_some() && parser.peek_as::<Comma>(cursor).unwrap_or(false) {
            let _ = parser.parse_as::<Comma>(cursor)?;
        }

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
            self_param,
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

    use crate::{TokenParser, stmt::ImplMethod};

    #[test]
    fn should_parse() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(b"fn stuff(self) { }").cursor();
        let ty = parser.parse_as::<ImplMethod>(&mut cursor)?;

        debug_assert_eq!(ty.to_string(), "fn stuff(self) { }");

        Ok(())
    }

    #[test]
    fn should_parse_static() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(b"fn new() -> Self { Self }").cursor();
        let ty = parser.parse_as::<ImplMethod>(&mut cursor)?;

        debug_assert_eq!(ty.to_string(), "fn new() -> Self { Self }");

        Ok(())
    }

    #[test]
    fn should_parse_with_visibility() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(b"pub(super) fn stuff(self) { }").cursor();
        let ty = parser.parse_as::<ImplMethod>(&mut cursor)?;

        debug_assert_eq!(ty.to_string(), "pub(super) fn stuff(self) { }");

        Ok(())
    }

    #[test]
    fn should_parse_with_return_type() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(b"fn stuff(self) -> models::User { }").cursor();
        let ty = parser.parse_as::<ImplMethod>(&mut cursor)?;

        debug_assert_eq!(ty.to_string(), "fn stuff(self) -> models::User { }");

        Ok(())
    }

    #[test]
    fn should_parse_with_params() -> Result<()> {
        let mut parser = TokenParser;
        let mut cursor = Span::from_bytes(b"fn stuff(&mut self, a: string, b: u32) { }").cursor();
        let ty = parser.parse_as::<ImplMethod>(&mut cursor)?;

        debug_assert_eq!(ty.to_string(), "fn stuff(&mut self, a: string, b: u32) { }");
        debug_assert_eq!(ty.params.len(), 2);

        Ok(())
    }
}
