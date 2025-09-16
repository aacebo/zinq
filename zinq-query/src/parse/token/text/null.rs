use std::fmt;

use crate::parse::{Position, bytes::Scanner};

/// ## Null Token
///
/// Example
/// -------
/// `null`
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Null {
    pub position: Position,
}

impl Null {
    pub fn parse(scan: &mut Scanner<'_>) -> Result<super::Text, crate::ParseError> {
        if scan.as_bytes() != b"null" {
            return Err(crate::ParseError::new("expected 'null'").position(scan.position()));
        }

        scan.commit();

        return Ok(super::Text::Null(Self {
            position: scan.position(),
        }));
    }
}

impl fmt::Display for Null {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "null");
    }
}

impl PartialEq<super::Text> for Null {
    fn eq(&self, other: &super::Text) -> bool {
        return match other {
            super::Text::Null(v) => v.eq(self),
            _ => false,
        };
    }
}

impl Into<super::Text> for Null {
    fn into(self) -> super::Text {
        return super::Text::Null(self.clone());
    }
}

impl Into<crate::parse::token::Token> for Null {
    fn into(self) -> crate::parse::token::Token {
        return crate::parse::token::Token::Text(self.into());
    }
}
