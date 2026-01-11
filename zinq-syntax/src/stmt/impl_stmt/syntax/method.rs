use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::{Comma, Fn, Ident, LParen, Punctuated, RArrow, RParen, Suffixed};

use crate::{
    Generics, Node, Visibility,
    meta::Meta,
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
    pub meta: Option<Meta>,
    pub vis: Visibility,
    pub keyword: Fn,
    pub name: Ident,
    pub generics: Option<Generics>,
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
        write!(f, "{}", self.span())
    }
}

impl Peek for ImplMethod {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        fork_parser.parse::<Option<Meta>>(&mut fork)?;
        fork_parser.parse::<Visibility>(&mut fork)?;
        Ok(fork_parser.peek::<Fn>(&fork).unwrap_or(false))
    }
}

impl Parse for ImplMethod {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let meta = parser.parse::<Option<Meta>>(cursor)?;
        let vis = parser.parse::<Visibility>(cursor)?;
        let keyword = parser.parse::<Fn>(cursor)?;
        let name = parser.parse::<Ident>(cursor)?;
        let generics = parser.parse::<Option<Generics>>(cursor)?;
        let left_paren = parser.parse::<LParen>(cursor)?;
        let self_param = parser.parse::<Option<SelfParam>>(cursor)?;

        if self_param.is_some() && parser.peek::<Comma>(cursor).unwrap_or(false) {
            let _ = parser.parse::<Comma>(cursor)?;
        }

        let params = parser.parse::<Punctuated<FnParam, Comma>>(cursor)?;
        let right_paren = parser.parse::<RParen>(cursor)?;
        let return_ty = parser.parse::<Option<Suffixed<RArrow, Type>>>(cursor)?;
        let block = parser.parse::<BlockStmt>(cursor)?;

        Ok(Self {
            meta,
            vis,
            keyword,
            name,
            generics,
            left_paren,
            self_param,
            params,
            right_paren,
            return_ty,
            block,
        })
    }
}

impl Spanned for ImplMethod {
    fn span(&self) -> Span {
        if let Some(meta) = &self.meta {
            return Span::join(meta.span(), self.block.span());
        }

        Span::join(self.vis.span(), self.block.span())
    }
}

#[cfg(test)]
mod test {
    use zinq_error::Result;
    use zinq_parse::Span;

    use crate::stmt::ImplMethod;

    #[test]
    fn should_parse() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"fn stuff(self) { }").cursor();
        let ty = parser.parse::<ImplMethod>(&mut cursor)?;

        debug_assert_eq!(ty.to_string(), "fn stuff(self) { }");

        Ok(())
    }

    #[test]
    fn should_parse_static() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"fn new() -> Self { Self }").cursor();
        let ty = parser.parse::<ImplMethod>(&mut cursor)?;

        debug_assert_eq!(ty.to_string(), "fn new() -> Self { Self }");

        Ok(())
    }

    #[test]
    fn should_parse_with_visibility() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"pub(super) fn stuff(self) { }").cursor();
        let ty = parser.parse::<ImplMethod>(&mut cursor)?;

        debug_assert_eq!(ty.to_string(), "pub(super) fn stuff(self) { }");

        Ok(())
    }

    #[test]
    fn should_parse_with_return_type() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"fn stuff(self) -> models::User { }").cursor();
        let ty = parser.parse::<ImplMethod>(&mut cursor)?;

        debug_assert_eq!(ty.to_string(), "fn stuff(self) -> models::User { }");

        Ok(())
    }

    #[test]
    fn should_parse_with_params() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(b"fn stuff(&mut self, a: string, b: u32) { }").cursor();
        let ty = parser.parse::<ImplMethod>(&mut cursor)?;

        debug_assert_eq!(ty.to_string(), "fn stuff(&mut self, a: string, b: u32) { }");
        debug_assert_eq!(ty.params.len(), 2);

        Ok(())
    }

    #[test]
    fn should_parse_with_meta() -> Result<()> {
        let mut parser = zinq_parse::ZinqParser;
        let mut cursor = Span::from_bytes(
            b"#[get(\"/users\")]
            fn get_users(&self) { }",
        )
        .cursor();

        let ty = parser.parse::<ImplMethod>(&mut cursor)?;

        debug_assert_eq!(
            ty.to_string(),
            "#[get(\"/users\")]
            fn get_users(&self) { }"
        );
        debug_assert!(ty.meta.is_some());
        debug_assert!(ty.params.is_empty());

        Ok(())
    }
}
