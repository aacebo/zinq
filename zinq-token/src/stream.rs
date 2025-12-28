use std::ops::Index;

use zinq_error::Result;

use zinq_parse::{Cursor, EOF, Parser, Span};

use crate::{Token, TokenParser};

#[derive(Debug, Clone)]
pub struct TokenStream {
    cursor: Cursor,
    parser: TokenParser,
    items: Vec<Token>,
}

impl TokenStream {
    pub fn from_str(text: &str) -> Self {
        let span = Span::from_str(text);

        Self {
            cursor: span.cursor(),
            parser: TokenParser,
            items: vec![],
        }
    }

    pub fn from_file(path: &str) -> Result<Self> {
        let span = Span::from_file(path)?;

        Ok(Self {
            cursor: span.cursor(),
            parser: TokenParser,
            items: vec![],
        })
    }

    pub fn next(&mut self) -> Result<Token> {
        if self.cursor.eof() {
            return Err(self.cursor.error(EOF, "End Of File"));
        }

        let value = self.parser.parse(&mut self.cursor)?;
        self.items.push(value.clone());
        Ok(value)
    }
}

impl std::fmt::Display for TokenStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for item in &self.items {
            write!(f, "{}", item)?;
        }

        Ok(())
    }
}

impl std::ops::Deref for TokenStream {
    type Target = [Token];

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl Eq for TokenStream {}

impl PartialEq for TokenStream {
    fn eq(&self, other: &Self) -> bool {
        if self.items.len() != other.items.len() {
            return false;
        }

        for i in 0..self.items.len() {
            if self.items.index(i) != other.items.index(i) {
                return false;
            }
        }

        true
    }
}
