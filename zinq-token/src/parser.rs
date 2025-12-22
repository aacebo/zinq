use zinq_error::Result;
use zinq_parse::{Cursor, Parse, Parser, Tx};

use crate::{Keyword, Token};

#[derive(Debug, Clone)]
pub struct TokenParser;

impl TokenParser {
    ///
    /// ## is_keyword
    /// `peek` at the next bytes to see if
    /// the next token should be parsed as
    /// a `Keyword`
    ///
    pub fn is_keyword(&self, cursor: &Cursor) -> bool {
        self.peek_as::<Keyword>(cursor)
    }

    ///
    /// ## keyword
    /// `parse` a `Keyword` token
    ///
    pub fn keyword(&mut self, cursor: &mut Cursor) -> Result<Token> {
        self.parse_as::<Keyword>(cursor)
    }
}

impl Parser for TokenParser {
    type Item = Token;
}
