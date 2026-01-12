use zinq_error::Result;
use zinq_parse::{Cursor, ZinqParser};
use zinq_token::{And, Comma, Ident, LParen, Mut, Or, Suffixed};

use crate::pat::*;

pub trait PatternParser {
    fn parse_pattern(&mut self, cursor: &mut Cursor) -> Result<Pattern>;
    fn parse_or_pattern(&mut self, cursor: &mut Cursor) -> Result<Pattern>;
    fn parse_tuple_pattern(&mut self, cursor: &mut Cursor) -> Result<Pattern>;
    fn parse_struct_pattern(&mut self, cursor: &mut Cursor) -> Result<Pattern>;
    fn parse_ref_pattern(&mut self, cursor: &mut Cursor) -> Result<Pattern>;
    fn parse_primary_pattern(&mut self, cursor: &mut Cursor) -> Result<Pattern>;
}

impl PatternParser for ZinqParser {
    fn parse_pattern(&mut self, cursor: &mut Cursor) -> Result<Pattern> {
        self.parse_or_pattern(cursor)
    }

    fn parse_or_pattern(&mut self, cursor: &mut Cursor) -> Result<Pattern> {
        let mut pat = self.parse_tuple_pattern(cursor)?;

        while self.peek::<Or>(cursor).unwrap_or(false) {
            let or = self.parse::<Or>(cursor)?;
            let right = self.parse_tuple_pattern(cursor)?;

            pat = OrPattern {
                left: Box::new(pat),
                or,
                right: Box::new(right),
            }
            .into();
        }

        Ok(pat)
    }

    fn parse_tuple_pattern(&mut self, cursor: &mut Cursor) -> Result<Pattern> {
        if self
            .peek::<Suffixed<LParen, Suffixed<Ident, Comma>>>(cursor)
            .unwrap_or(false)
        {
            return Ok(self.parse::<TuplePattern>(cursor)?.into());
        }

        self.parse_struct_pattern(cursor)
    }

    fn parse_struct_pattern(&mut self, cursor: &mut Cursor) -> Result<Pattern> {
        if self.peek::<StructPattern>(cursor).unwrap_or(false) {
            return Ok(self.parse::<StructPattern>(cursor)?.into());
        }

        self.parse_primary_pattern(cursor)
    }

    fn parse_ref_pattern(&mut self, cursor: &mut Cursor) -> Result<Pattern> {
        if self.peek::<And>(cursor).unwrap_or(false) {
            let and = self.parse::<And>(cursor)?;
            let mutable = self.parse::<Option<Mut>>(cursor)?;
            let inner = self.parse_ref_pattern(cursor)?;

            return Ok(RefPattern {
                and,
                mutable,
                inner: Box::new(inner),
            }
            .into());
        }

        self.parse_tuple_pattern(cursor)
    }

    fn parse_primary_pattern(&mut self, cursor: &mut Cursor) -> Result<Pattern> {
        if self.peek::<WildPattern>(cursor).unwrap_or(false) {
            return Ok(self.parse::<WildPattern>(cursor)?.into());
        }

        if self.peek::<SpreadPattern>(cursor).unwrap_or(false) {
            return Ok(self.parse::<SpreadPattern>(cursor)?.into());
        }

        if self.peek::<LiteralPattern>(cursor).unwrap_or(false) {
            return Ok(self.parse::<LiteralPattern>(cursor)?.into());
        }

        if self.peek::<PathPattern>(cursor).unwrap_or(false) {
            return Ok(self.parse::<PathPattern>(cursor)?.into());
        }

        if self.peek::<GroupPattern>(cursor).unwrap_or(false) {
            return Ok(self.parse::<GroupPattern>(cursor)?.into());
        }

        Err(cursor.error(
            zinq_error::NOT_FOUND,
            &format!("unexpected token '{}'", *cursor.peek()? as char),
        ))
    }
}
