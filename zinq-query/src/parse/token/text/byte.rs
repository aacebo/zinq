use std::fmt;

use crate::parse::{Position, bytes::Scanner};

/// ## Byte Token
///
/// Example
/// -------
/// `'a'`
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Byte {
    pub position: Position,
    pub value: u8,
}

impl Byte {
    pub fn parse(scan: &mut Scanner<'_>) -> Result<super::Text, crate::ParseError> {
        let value = scan.curr();
        scan.commit();

        return Ok(super::Text::Byte(Self {
            position: scan.position(),
            value,
        }));
    }
}

impl fmt::Display for Byte {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.value);
    }
}

impl PartialEq<super::Text> for Byte {
    fn eq(&self, other: &super::Text) -> bool {
        return match other {
            super::Text::Byte(v) => v.eq(self),
            _ => false,
        };
    }
}

impl Into<super::Text> for Byte {
    fn into(self) -> super::Text {
        return super::Text::Byte(self.clone());
    }
}

impl Into<crate::parse::token::Token> for Byte {
    fn into(self) -> crate::parse::token::Token {
        return crate::parse::token::Token::Text(self.into());
    }
}
