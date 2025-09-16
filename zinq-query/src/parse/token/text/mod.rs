mod bool;
mod byte;
mod ident;
mod null;
mod string;

pub use bool::*;
pub use byte::*;
pub use ident::*;
pub use null::*;
pub use string::*;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Text {
    Bool(Bool),
    Byte(Byte),
    Ident(Ident),
    Null(Null),
    String(String),
}

impl Text {
    pub fn parse(
        scan: &mut crate::parse::bytes::Scanner<'_>,
    ) -> Result<super::Token, crate::ParseError> {
        return match scan.curr() {
            b'"' => Ok(super::Token::Text(String::parse(scan)?)),
            b'\'' => Ok(super::Token::Text(Byte::parse(scan)?)),
            b => {
                if !b.is_ascii_alphabetic() {
                    return Err(crate::ParseError::new(&format!(
                        "unexpected syntax error at '{}'",
                        b
                    )));
                }

                while !scan.is_eof() && scan.peek().unwrap_or(0).is_ascii_alphanumeric() {
                    scan.next();
                }

                let position = scan.position();

                Ok(super::Token::Text(match scan.commit() {
                    "true" => Self::Bool(Bool {
                        position,
                        value: true,
                    }),
                    "false" => Self::Bool(Bool {
                        position,
                        value: false,
                    }),
                    "null" => Self::Null(Null { position }),
                    value => Self::Ident(Ident {
                        position,
                        name: value.to_string(),
                    }),
                }))
            }
        };
    }
}

impl Text {
    pub fn is_bool(&self) -> bool {
        return match self {
            Self::Bool(_) => true,
            _ => false,
        };
    }

    pub fn is_byte(&self) -> bool {
        return match self {
            Self::Byte(_) => true,
            _ => false,
        };
    }

    pub fn is_ident(&self) -> bool {
        return match self {
            Self::Ident(_) => true,
            _ => false,
        };
    }

    pub fn is_null(&self) -> bool {
        return match self {
            Self::Null(_) => true,
            _ => false,
        };
    }

    pub fn is_string(&self) -> bool {
        return match self {
            Self::String(_) => true,
            _ => false,
        };
    }
}

impl std::fmt::Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Bool(v) => write!(f, "{}", v),
            Self::Byte(v) => write!(f, "{}", v),
            Self::Ident(v) => write!(f, "{}", v),
            Self::Null(v) => write!(f, "{}", v),
            Self::String(v) => write!(f, "{}", v),
        };
    }
}

impl PartialEq<super::Token> for Text {
    fn eq(&self, other: &super::Token) -> bool {
        return match other {
            super::Token::Text(v) => v.eq(self),
            _ => false,
        };
    }
}

impl Into<super::Token> for Text {
    fn into(self) -> crate::parse::token::Token {
        return super::Token::Text(self.clone());
    }
}
