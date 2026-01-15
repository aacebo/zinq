use zinq_parse::{Parse, Peek, Span, Spanned};

use crate::Visibility;

///
/// ## Private Visibility
/// `test: string`
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PrivateVisibility {
    pub span: Span,
}

impl From<PrivateVisibility> for Visibility {
    fn from(value: PrivateVisibility) -> Self {
        Self::Priv(value)
    }
}

impl std::fmt::Display for PrivateVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek for PrivateVisibility {
    fn peek(_: &zinq_parse::Cursor, _: &zinq_parse::ZinqParser) -> zinq_error::Result<bool> {
        Ok(true)
    }
}

impl Parse for PrivateVisibility {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        _: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        Ok(Self {
            span: cursor.span().clone(),
        })
    }
}

impl Spanned for PrivateVisibility {
    fn span(&self) -> Span {
        self.span.clone()
    }
}
