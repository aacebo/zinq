use zinq_parse::{Parse, Parser, Peek};
use zinq_token::TokenParser;

use crate::param::{FnParam, SelfParam};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MethodParam {
    Fn(FnParam),
    Receiver(SelfParam),
}

impl std::fmt::Display for MethodParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fn(v) => write!(f, "{}", v),
            Self::Receiver(v) => write!(f, "{}", v),
        }
    }
}

impl Peek<TokenParser> for MethodParam {
    fn peek(cursor: &zinq_parse::Cursor, parser: &TokenParser) -> zinq_error::Result<bool> {
        let mut fork = cursor.fork();
        let mut fork_parser = parser.clone();

        match fork_parser.parse_as::<Self>(&mut fork) {
            Err(_) => Ok(false),
            Ok(_) => Ok(true),
        }
    }
}

impl Parse<TokenParser> for MethodParam {
    fn parse(
        cursor: &mut zinq_parse::Cursor,
        parser: &mut TokenParser,
    ) -> zinq_error::Result<Self> {
        if parser.peek_as::<SelfParam>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<SelfParam>(cursor)?.into());
        }

        if parser.peek_as::<FnParam>(cursor).unwrap_or(false) {
            return Ok(parser.parse_as::<FnParam>(cursor)?.into());
        }

        Err(cursor.error(
            zinq_error::NOT_FOUND,
            &format!("unknown tokens '{}'", cursor),
        ))
    }

    fn span(&self) -> &zinq_parse::Span {
        match self {
            Self::Fn(v) => v.span(),
            Self::Receiver(v) => v.span(),
        }
    }
}
