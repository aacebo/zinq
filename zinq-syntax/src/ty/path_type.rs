use zinq_parse::{Parse, Parser, Peek, Span};
use zinq_token::{ColonColon, Ident, Punctuated, TokenParser};

use crate::{Node, ty::Type};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathType {
    pub span: Span,
    pub path: Punctuated<Ident, ColonColon>,
}

impl From<PathType> for Type {
    fn from(value: PathType) -> Self {
        Type::Path(value)
    }
}

impl Node for PathType {
    fn name(&self) -> &str {
        "Syntax::Type::Path"
    }

    fn accept<V: crate::Visitor<Self>>(&self, visitor: &mut V) -> zinq_error::Result<()>
    where
        Self: Sized,
    {
        visitor.visit(self)
    }
}

impl std::fmt::Display for PathType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for PathType {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse<TokenParser> for PathType {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        let path = parser.parse_as::<Punctuated<Ident, ColonColon>>(cursor)?;

        Ok(Self {
            span: path.span().clone(),
            path,
        })
    }

    fn span(&self) -> &Span {
        &self.span
    }
}
