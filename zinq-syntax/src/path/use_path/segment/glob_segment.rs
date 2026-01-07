use zinq_parse::{Parse, Peek, Span};
use zinq_token::Star;

use crate::path::UseSegment;

///
/// ## Use Glob
/// `use std::string::*;`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseGlob {
    pub star: Star,
}

impl From<UseGlob> for UseSegment {
    fn from(value: UseGlob) -> Self {
        Self::Glob(value)
    }
}

impl std::fmt::Display for UseGlob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.star.span())
    }
}

impl Peek for UseGlob {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser.peek::<Star>(cursor).unwrap_or(false))
    }
}

impl Parse for UseGlob {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let star = parser.parse::<Star>(cursor)?;

        Ok(Self { star })
    }

    fn span(&self) -> &Span {
        self.star.span()
    }
}
