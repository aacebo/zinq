mod method;

pub use method::*;
use zinq_parse::{Parse, Parser, Peek};
use zinq_token::TokenParser;

#[derive(Debug, Clone, PartialEq, Eq)]
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

impl Peek<TokenParser> for ImplSyntax {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse<TokenParser> for ImplSyntax {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        if parser.peek_as::<ImplMethod>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<ImplMethod>(cursor)?.into());
        }

        Err(cursor.error(
            zinq_error::NOT_FOUND,
            &format!("unknown tokens '{}'", cursor),
        ))
    }

    fn span(&self) -> &zinq_parse::Span {
        match self {
            Self::Method(v) => v.span(),
        }
    }
}
