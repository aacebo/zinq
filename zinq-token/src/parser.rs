use zinq_parse::{Cursor, Parser};

use crate::Token;

#[derive(Debug, Clone)]
pub struct TokenParser {
    cursor: Cursor,
}

impl Parser for TokenParser {
    type Item = Token;

    fn cursor(&self) -> &Cursor {
        &self.cursor
    }

    fn cursor_mut(&mut self) -> &mut Cursor {
        &mut self.cursor
    }
}
