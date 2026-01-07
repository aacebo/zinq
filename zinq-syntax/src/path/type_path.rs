use zinq_parse::{Parse, Peek, Span, Spanned};
use zinq_token::{ColonColon, Ident, Punctuated};

use crate::{Node, Visitor, ty::Type};

///
/// ## Type Path
/// `std::string::String`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypePath {
    pub path: Punctuated<Ident, ColonColon>,
}

impl From<TypePath> for Type {
    fn from(value: TypePath) -> Self {
        Self::Path(value)
    }
}

impl Node for TypePath {
    fn name(&self) -> &str {
        "TypePath"
    }

    fn accept<V: Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for TypePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.span())
    }
}

impl Peek for TypePath {
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

impl Parse for TypePath {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        let path = parser.parse::<Punctuated<Ident, ColonColon>>(cursor)?;
        Ok(Self { path })
    }
}

impl Spanned for TypePath {
    fn span(&self) -> Span {
        self.path.span()
    }
}
