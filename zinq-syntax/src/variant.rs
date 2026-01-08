use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::{Eq, Ident};

use crate::{expr::Expr, fields::Fields};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variant {
    pub name: Ident,
    pub fields: Fields,
    pub discriminant: Option<(Eq, Expr)>,
}

impl std::fmt::Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Peek for Variant {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser.peek::<Ident>(cursor).unwrap_or(false))
    }
}

impl Parse for Variant {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let name = parser.parse::<Ident>(cursor)?;
        let fields = parser.parse::<Fields>(cursor)?;
        let mut discriminant = None;

        if parser.peek::<Eq>(cursor).unwrap_or(false) {
            let eq = parser.parse::<Eq>(cursor)?;
            let expr = parser.parse::<Expr>(cursor)?;
            discriminant = Some((eq, expr));
        }

        Ok(Self {
            name,
            fields,
            discriminant,
        })
    }
}

impl Spanned for Variant {
    fn span(&self) -> zinq_parse::Span {
        if let Some(v) = &self.discriminant {
            return Span::join(self.name.span(), v.1.span());
        }

        Span::join(self.name.span(), self.fields.span())
    }
}
