use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::{ColonColon, Ident, Suffixed};

use crate::Generics;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathSection {
    pub ident: Ident,
    pub generics: Option<Suffixed<ColonColon, Generics>>,
}

impl std::fmt::Display for PathSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Spanned for PathSection {
    fn span(&self) -> zinq_parse::Span {
        if let Some(generics) = &self.generics {
            return Span::join(self.ident.span(), generics.span());
        }

        self.ident.span()
    }
}

impl Peek for PathSection {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser.peek::<Ident>(cursor).unwrap_or(false))
    }
}

impl Parse for PathSection {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let ident = parser.parse::<Ident>(cursor)?;
        let generics = parser.parse::<Option<Suffixed<ColonColon, Generics>>>(cursor)?;

        Ok(Self { ident, generics })
    }
}
