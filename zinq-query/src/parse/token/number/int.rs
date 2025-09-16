use std::fmt;

use crate::parse::Position;

/// ## Int Token
///
/// Example
/// -------
/// `1`
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Int {
    pub position: Position,
    pub value: i64,
}

impl fmt::Display for Int {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.value);
    }
}

impl PartialEq<super::Number> for Int {
    fn eq(&self, other: &super::Number) -> bool {
        return match other {
            super::Number::Int(v) => v.eq(self),
            _ => false,
        };
    }
}

impl Into<super::Number> for Int {
    fn into(self) -> super::Number {
        return super::Number::Int(self.clone());
    }
}

impl Into<crate::parse::token::Token> for Int {
    fn into(self) -> crate::parse::token::Token {
        return crate::parse::token::Token::Number(self.into());
    }
}
