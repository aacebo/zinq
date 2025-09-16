mod bytes;
mod position;

pub mod expression;
pub mod token;

pub(crate) use bytes::*;
pub(crate) use position::*;

use std::collections::BTreeMap;

impl crate::Query {
    pub fn parse(text: &str) -> Result<Self, crate::ParseError> {
        let mut scan = Scanner::from(text);
        let _ = expression::Expression::parse(&mut scan)?;

        return Ok(Self {
            args: zinq_schema::value::Object::new(),
            fields: BTreeMap::new(),
        });
    }
}
