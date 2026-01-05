use zinq_parse::{Parse, Parser, Peek, Span};
use zinq_token::{Colon, Ident, TokenParser};

use crate::{param::Param, ty::Type};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FnParam {
    pub span: Span,
    pub name: Ident,
    pub colon: Colon,
    pub ty: Type,
}

impl From<FnParam> for Param {
    fn from(value: FnParam) -> Self {
        Self::Fn(value)
    }
}

impl std::fmt::Display for FnParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for FnParam {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse<TokenParser> for FnParam {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        let name = parser.parse_as::<Ident>(cursor)?;
        let colon = parser.parse_as::<Colon>(cursor)?;
        let ty = parser.parse_as::<Type>(cursor)?;
        let span = Span::from_bounds(name.span(), ty.span());

        Ok(Self {
            span,
            name,
            colon,
            ty,
        })
    }

    fn span(&self) -> &Span {
        &self.span
    }
}
