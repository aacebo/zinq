use zinq_error::Result;
use zinq_parse::{Cursor, ZinqParser};

use crate::pat::*;

pub trait PatternParser {
    fn parse_pattern(&mut self, cursor: &mut Cursor) -> Result<Pattern>;
    fn parse_prefix_pattern(&mut self, cursor: &mut Cursor) -> Result<Pattern>;
    fn parse_object_pattern(&mut self, cursor: &mut Cursor) -> Result<Pattern>;
    fn parse_primary_pattern(&mut self, cursor: &mut Cursor) -> Result<Pattern>;
}

impl PatternParser for ZinqParser {
    fn parse_pattern(&mut self, cursor: &mut Cursor) -> Result<Pattern> {
        self.parse_object_pattern(cursor)
    }

    fn parse_prefix_pattern(&mut self, cursor: &mut Cursor) -> Result<Pattern> {
        if self.peek::<RefPattern>(cursor).unwrap_or(false) {
            return Ok(self.parse::<RefPattern>(cursor)?.into());
        }

        self.parse_object_pattern(cursor)
    }

    fn parse_object_pattern(&mut self, cursor: &mut Cursor) -> Result<Pattern> {
        if self.peek::<TuplePattern>(cursor).unwrap_or(false) {
            return Ok(self.parse::<TuplePattern>(cursor)?.into());
        }

        if self.peek::<StructPattern>(cursor).unwrap_or(false) {
            return Ok(self.parse::<StructPattern>(cursor)?.into());
        }

        self.parse_primary_pattern(cursor)
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

        if self.peek::<TypePattern>(cursor).unwrap_or(false) {
            return Ok(self.parse::<TypePattern>(cursor)?.into());
        }

        Err(cursor.error(
            zinq_error::NOT_FOUND,
            &format!("unknown tokens '{}'", cursor),
        ))
    }
}
