use std::fmt;

use crate::parse::{Position, bytes::Scanner};

/// ## String Token
///
/// Example
/// -------
/// `"test"`
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct String {
    pub position: Position,
    pub value: std::string::String,
}

impl String {
    pub fn parse(scan: &mut Scanner<'_>) -> Result<super::Text, crate::ParseError> {
        if !scan.next_if(b'"') {
            return Err(crate::ParseError::new("expected '\"'").position(scan.position()));
        }

        while !scan.is_eof() && scan.peek().unwrap_or(0).is_ascii_alphanumeric() {
            scan.next();
        }

        if !scan.next_if(b'"') {
            return Err(crate::ParseError::new("expected '\"'").position(scan.position()));
        }

        return Ok(super::Text::String(Self {
            position: scan.position(),
            value: scan.commit().to_string(),
        }));
    }
}

impl fmt::Display for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.value);
    }
}

impl PartialEq<super::Text> for String {
    fn eq(&self, other: &super::Text) -> bool {
        return match other {
            super::Text::String(v) => v.eq(self),
            _ => false,
        };
    }
}

impl Into<super::Text> for String {
    fn into(self) -> super::Text {
        return super::Text::String(self.clone());
    }
}

impl Into<crate::parse::token::Token> for String {
    fn into(self) -> crate::parse::token::Token {
        return crate::parse::token::Token::Text(self.into());
    }
}
