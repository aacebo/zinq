use zinq_parse::{Parse, Peek, Span};
use zinq_token::{Colon, Ident};

use crate::ty::Type;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FnParam {
    pub span: Span,
    pub name: Ident,
    pub colon: Colon,
    pub ty: Type,
}

impl std::fmt::Display for FnParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek for FnParam {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse for FnParam {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let name = parser.parse::<Ident>(cursor)?;
        let colon = parser.parse::<Colon>(cursor)?;
        let ty = parser.parse::<Type>(cursor)?;
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
