use std::fmt;

use crate::parse::{Position, bytes::Scanner};

/// ## Bool Token
///
/// Example
/// -------
/// `null`
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Bool {
    pub position: Position,
    pub value: bool,
}

impl Bool {
    pub fn parse(scan: &mut Scanner<'_>) -> Result<super::Text, crate::ParseError> {
        let bytes = scan.to_string();

        if bytes != "true" && bytes != "false" {
            return Err(
                crate::ParseError::new("expected 'true' or 'false'").position(scan.position())
            );
        }

        scan.commit();

        return Ok(super::Text::Bool(Self {
            position: scan.position(),
            value: bytes == "true",
        }));
    }
}

impl fmt::Display for Bool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.value);
    }
}

impl PartialEq<super::Text> for Bool {
    fn eq(&self, other: &super::Text) -> bool {
        return match other {
            super::Text::Bool(v) => v.eq(self),
            _ => false,
        };
    }
}

impl Into<super::Text> for Bool {
    fn into(self) -> super::Text {
        return super::Text::Bool(self.clone());
    }
}

impl Into<crate::parse::token::Token> for Bool {
    fn into(self) -> crate::parse::token::Token {
        return crate::parse::token::Token::Text(self.into());
    }
}
