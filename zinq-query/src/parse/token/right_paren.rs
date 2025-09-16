use std::fmt;

use crate::parse::{Position, bytes::Scanner};

/// ## RightParen Token
///
/// Example
/// -------
/// `)`
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RightParen {
    pub position: Position,
}

impl RightParen {
    pub fn parse(scan: &mut Scanner<'_>) -> Result<super::Token, crate::ParseError> {
        if scan.curr() != b')' {
            return Err(crate::ParseError::new("expected ')'").position(scan.position()));
        }

        let position = scan.position();
        scan.commit();

        return Ok(super::Token::RightParen(Self { position }));
    }
}

impl fmt::Display for RightParen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, ")");
    }
}

impl PartialEq<super::Token> for RightParen {
    fn eq(&self, other: &super::Token) -> bool {
        return match other {
            super::Token::RightParen(v) => v.eq(self),
            _ => false,
        };
    }
}

impl Into<super::Token> for RightParen {
    fn into(self) -> super::Token {
        return super::Token::RightParen(self.clone());
    }
}
