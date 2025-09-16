use std::fmt;

use crate::parse::Position;

/// ## Float Token
///
/// Example
/// -------
/// `1.2`
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Float {
    pub position: Position,
    pub value: f64,
}

impl fmt::Display for Float {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.value);
    }
}

impl PartialEq<super::Number> for Float {
    fn eq(&self, other: &super::Number) -> bool {
        return match other {
            super::Number::Float(v) => v.eq(self),
            _ => false,
        };
    }
}

impl Into<super::Number> for Float {
    fn into(self) -> super::Number {
        return super::Number::Float(self.clone());
    }
}

impl Into<crate::parse::token::Token> for Float {
    fn into(self) -> crate::parse::token::Token {
        return crate::parse::token::Token::Number(self.into());
    }
}
