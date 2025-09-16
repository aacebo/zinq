use std::fmt;

use crate::parse::{Position, bytes::Scanner};

/// ## Colon Token
///
/// Example
/// -------
/// `:`
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Colon {
    pub position: Position,
}

impl Colon {
    pub fn parse(scan: &mut Scanner<'_>) -> Result<super::Token, crate::ParseError> {
        if scan.curr() != b':' {
            return Err(crate::ParseError::new("expected ':'").position(scan.position()));
        }

        let position = scan.position();
        scan.commit();

        return Ok(super::Token::Colon(Self { position }));
    }
}

impl fmt::Display for Colon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, ":");
    }
}

impl PartialEq<super::Token> for Colon {
    fn eq(&self, other: &super::Token) -> bool {
        return match other {
            super::Token::Colon(v) => v.eq(self),
            _ => false,
        };
    }
}

impl Into<super::Token> for Colon {
    fn into(self) -> super::Token {
        return super::Token::Colon(self.clone());
    }
}
