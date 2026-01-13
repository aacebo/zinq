use zinq_parse::{Parse, Peek, Spanned};

use crate::{Node, Path, pat::Pattern};

///
/// ## Path Pattern
/// - `a`
/// - `std::string::String`
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathPattern {
    pub path: Path,
}

impl From<PathPattern> for Pattern {
    fn from(value: PathPattern) -> Self {
        Self::Path(value)
    }
}

impl std::fmt::Display for PathPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Spanned for PathPattern {
    fn span(&self) -> zinq_parse::Span {
        self.path.span()
    }
}

impl Peek for PathPattern {
    fn peek(
        cursor: &zinq_parse::Cursor,
        parser: &zinq_parse::ZinqParser,
    ) -> zinq_error::Result<bool> {
        Ok(parser.peek::<Path>(cursor).unwrap_or(false))
    }
}

impl Parse for PathPattern {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let path = parser.parse::<Path>(cursor)?;
        Ok(Self { path })
    }
}

impl Node for PathPattern {
    fn name(&self) -> &str {
        "Pattern::Path"
    }

    fn accept<V: crate::Visitor>(&self, visitor: &mut V) {
        visitor.visit_path_pattern(self);
    }
}
