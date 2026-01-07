use zinq_parse::{Parse, Peek, Span};
use zinq_token::{ColonColon, Ident, Punctuated};

use crate::{Node, Visitor, ty::Type};

///
/// ## Path
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Path {
    pub span: Span,
    pub path: Punctuated<Ident, ColonColon>,
}

impl From<Path> for Type {
    fn from(value: Path) -> Self {
        Self::Path(value)
    }
}

impl Node for Path {
    fn name(&self) -> &str {
        "Path"
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek for Path {
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

impl Parse for Path {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let path = parser.parse::<Punctuated<Ident, ColonColon>>(cursor)?;

        Ok(Self {
            span: path.span().clone(),
            path,
        })
    }

    fn span(&self) -> &Span {
        &self.span
    }
}
