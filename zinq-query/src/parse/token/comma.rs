use std::fmt;

use crate::parse::{Position, bytes::Scanner};

/// ## Comma Token
///
/// Example
/// -------
/// `,`
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Comma {
    pub position: Position,
}

impl Comma {
    pub fn parse(scan: &mut Scanner<'_>) -> Result<super::Token, crate::ParseError> {
        if scan.curr() != b',' {
            return Err(crate::ParseError::new("expected ','").position(scan.position()));
        }

        let position = scan.position();
        scan.commit();

        return Ok(super::Token::Comma(Self { position }));
    }
}

impl fmt::Display for Comma {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, ",");
    }
}

impl PartialEq<super::Token> for Comma {
    fn eq(&self, other: &super::Token) -> bool {
        return match other {
            super::Token::Comma(v) => v.eq(self),
            _ => false,
        };
    }
}

impl Into<super::Token> for Comma {
    fn into(self) -> super::Token {
        return super::Token::Comma(self.clone());
    }
}
