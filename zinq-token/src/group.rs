use zinq_error::Result;
use zinq_parse::{Cursor, Parse, Parser, Peek, Span};

use crate::{Keyword, Token, TokenParser};

///
/// ## Group
/// a dimimited sub-stream
/// ### Examples
/// - (...)
/// - {...}
/// - [...]
/// - <...>
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Group {
    span: Span,
}

impl Group {
    pub fn name(&self) -> &'static str {
        "Group"
    }
}

impl From<Group> for Token {
    #[inline]
    fn from(value: Group) -> Self {
        Self::Group(value)
    }
}

impl std::fmt::Display for Group {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.span)
    }
}

impl Peek<TokenParser> for Group {
    #[inline]
    fn peek(cursor: &Cursor, parser: &TokenParser) -> Result<bool> {
        match cursor.peek()? {
            b'(' | b'[' | b'{' => Ok(true),
            _ => Ok(false),
        }
    }
}

impl Parse<TokenParser> for Group {
    #[inline]
    fn parse(cursor: &mut Cursor, parser: &mut TokenParser) -> Result<Token> {
        todo!()
    }

    #[inline]
    fn span(&self) -> &Span {
        &self.span
    }
}
