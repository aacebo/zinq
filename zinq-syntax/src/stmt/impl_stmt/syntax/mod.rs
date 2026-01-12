mod method;

pub use method::*;
use zinq_parse::{Parse, Peek, Spanned};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ImplSyntax {
    Method(ImplMethod),
}

impl ImplSyntax {
    pub fn is_method(&self) -> bool {
        match self {
            Self::Method(_) => true,
        }
    }
}

impl std::fmt::Display for ImplSyntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Method(v) => write!(f, "{}", v),
        }
    }
}

impl Peek for ImplSyntax {
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

impl Parse for ImplSyntax {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut zinq_parse::ZinqParser,
    ) -> zinq_error::Result<Self> {
        if parser.peek::<ImplMethod>(cursor).unwrap_or(false) {
            return Ok(parser.parse::<ImplMethod>(cursor)?.into());
        }

        Err(cursor.error(
            zinq_error::NOT_FOUND,
            &format!("unknown tokens '{}'", cursor),
        ))
    }
}

impl Spanned for ImplSyntax {
    fn span(&self) -> zinq_parse::Span {
        match self {
            Self::Method(v) => v.span(),
        }
    }
}
