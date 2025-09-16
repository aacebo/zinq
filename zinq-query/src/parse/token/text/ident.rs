use std::fmt;

use crate::parse::Position;

/// ## Ident Token
///
/// Example
/// -------
/// `test`
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ident {
    pub position: Position,
    pub name: String,
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.name);
    }
}

impl PartialEq<super::Text> for Ident {
    fn eq(&self, other: &super::Text) -> bool {
        return match other {
            super::Text::Ident(v) => v.eq(self),
            _ => false,
        };
    }
}

impl Into<super::Text> for Ident {
    fn into(self) -> super::Text {
        return super::Text::Ident(self.clone());
    }
}

impl Into<crate::parse::token::Token> for Ident {
    fn into(self) -> crate::parse::token::Token {
        return crate::parse::token::Token::Text(self.into());
    }
}
