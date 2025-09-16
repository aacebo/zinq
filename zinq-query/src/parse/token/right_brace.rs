use std::fmt;

use crate::parse::{Position, bytes::Scanner};

/// ## RightBrace Token
///
/// Example
/// -------
/// `}`
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RightBrace {
    pub position: Position,
}

impl RightBrace {
    pub fn parse(scan: &mut Scanner<'_>) -> Result<super::Token, crate::ParseError> {
        if scan.curr() != b'}' {
            return Err(crate::ParseError::new("expected '}'").position(scan.position()));
        }

        let position = scan.position();
        scan.commit();

        return Ok(super::Token::RightBrace(Self { position }));
    }
}

impl fmt::Display for RightBrace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "}}");
    }
}

impl PartialEq<super::Token> for RightBrace {
    fn eq(&self, other: &super::Token) -> bool {
        return match other {
            super::Token::RightBrace(v) => v.eq(self),
            _ => false,
        };
    }
}

impl Into<super::Token> for RightBrace {
    fn into(self) -> super::Token {
        return super::Token::RightBrace(self.clone());
    }
}
