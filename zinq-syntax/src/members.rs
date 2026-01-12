use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::{Colon, Ident, LInt};

use crate::expr::Expr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Member {
    Index(LInt),
    Name(Ident),
}

impl std::fmt::Display for Member {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Index(v) => write!(f, "{}", v),
            Self::Name(v) => write!(f, "{}", v),
        }
    }
}

impl Spanned for Member {
    fn span(&self) -> zinq_parse::Span {
        match self {
            Self::Index(v) => v.span(),
            Self::Name(v) => v.span(),
        }
    }
}

impl Peek for Member {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser.peek::<LInt>(cursor).unwrap_or(false)
            || parser.peek::<Ident>(cursor).unwrap_or(false))
    }
}

impl Parse for Member {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        if parser.peek::<LInt>(cursor).unwrap_or(false) {
            return Ok(Self::Index(parser.parse::<LInt>(cursor)?));
        }

        if parser.peek::<Ident>(cursor).unwrap_or(false) {
            return Ok(Self::Name(parser.parse::<Ident>(cursor)?));
        }

        Err(cursor.error(
            zinq_error::NOT_FOUND,
            &format!("unexpected token '{}'", *cursor.peek()? as char),
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MemberValue {
    pub member: Member,
    pub colon: Option<Colon>,
    pub expr: Option<Box<Expr>>,
}

impl std::fmt::Display for MemberValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Spanned for MemberValue {
    fn span(&self) -> zinq_parse::Span {
        if let Some(expr) = &self.expr {
            return Span::join(self.member.span(), expr.span());
        }

        self.member.span()
    }
}

impl Peek for MemberValue {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser.peek::<Member>(cursor).unwrap_or(false))
    }
}

impl Parse for MemberValue {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let member = parser.parse::<Member>(cursor)?;
        let mut colon = None;
        let mut expr = None;

        if parser.peek::<Colon>(cursor).unwrap_or(false) {
            colon = Some(parser.parse::<Colon>(cursor)?);
            expr = Some(parser.parse::<Box<Expr>>(cursor)?);
        }

        Ok(Self {
            member,
            colon,
            expr,
        })
    }
}
