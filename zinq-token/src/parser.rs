use zinq_error::Result;
use zinq_parse::{Cursor, Parse, Parser, Tx};

use crate::{Group, Ident, Keyword, Literal, Punct, Token, TokenStream};

#[derive(Debug, Clone)]
pub struct TokenParser;

impl TokenParser {
    #[inline]
    pub fn is_group(&self, cursor: &Cursor) -> bool {
        self.peek_as::<Group>(cursor)
    }

    #[inline]
    pub fn group(&mut self, cursor: &mut Cursor) -> Result<Token> {
        self.parse_as::<Group>(cursor)
    }

    #[inline]
    pub fn is_punct(&self, cursor: &Cursor) -> bool {
        self.peek_as::<Punct>(cursor)
    }

    #[inline]
    pub fn punct(&mut self, cursor: &mut Cursor) -> Result<Token> {
        self.parse_as::<Punct>(cursor)
    }

    #[inline]
    pub fn is_literal(&self, cursor: &Cursor) -> bool {
        self.peek_as::<Literal>(cursor)
    }

    #[inline]
    pub fn literal(&mut self, cursor: &mut Cursor) -> Result<Token> {
        self.parse_as::<Literal>(cursor)
    }

    #[inline]
    pub fn is_keyword(&self, cursor: &Cursor) -> bool {
        self.peek_as::<Keyword>(cursor)
    }

    #[inline]
    pub fn keyword(&mut self, cursor: &mut Cursor) -> Result<Token> {
        self.parse_as::<Keyword>(cursor)
    }

    #[inline]
    pub fn is_ident(&self, cursor: &Cursor) -> bool {
        self.peek_as::<Ident>(cursor)
    }

    #[inline]
    pub fn ident(&mut self, cursor: &mut Cursor) -> Result<Token> {
        self.parse_as::<Ident>(cursor)
    }
}

impl Parser for TokenParser {
    type Item = Token;
}
