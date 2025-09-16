#![allow(unused)]

mod colon;
mod comma;
mod left_brace;
mod left_paren;
mod number;
mod right_brace;
mod right_paren;
mod text;

pub use colon::*;
pub use comma::*;
pub use left_brace::*;
pub use left_paren::*;
pub use number::*;
pub use right_brace::*;
pub use right_paren::*;
pub use text::*;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Token {
    Colon(Colon),
    Comma(Comma),
    LeftBrace(LeftBrace),
    LeftParen(LeftParen),
    Number(Number),
    RightBrace(RightBrace),
    RightParen(RightParen),
    Text(Text),
}

impl Token {
    pub fn parse(scan: &mut super::bytes::Scanner<'_>) -> Result<Self, crate::ParseError> {
        return match scan.curr() {
            b':' => Ok(Colon::parse(scan)?),
            b',' => Ok(Comma::parse(scan)?),
            b'{' => Ok(LeftBrace::parse(scan)?),
            b'(' => Ok(LeftParen::parse(scan)?),
            b'}' => Ok(RightBrace::parse(scan)?),
            b')' => Ok(RightParen::parse(scan)?),
            b => {
                if b.is_ascii_digit() {
                    return Ok(Number::parse(scan)?);
                }

                return Ok(Text::parse(scan)?);
            }
        };
    }
}

impl Token {
    pub fn is_bool(&self) -> bool {
        return match self {
            Self::Text(v) => v.is_bool(),
            _ => false,
        };
    }

    pub fn is_byte(&self) -> bool {
        return match self {
            Self::Text(v) => v.is_byte(),
            _ => false,
        };
    }

    pub fn is_colon(&self) -> bool {
        return match self {
            Self::Colon(_) => true,
            _ => false,
        };
    }

    pub fn is_comma(&self) -> bool {
        return match self {
            Self::Comma(_) => true,
            _ => false,
        };
    }

    pub fn is_ident(&self) -> bool {
        return match self {
            Self::Text(v) => v.is_ident(),
            _ => false,
        };
    }

    pub fn is_left_brace(&self) -> bool {
        return match self {
            Self::LeftBrace(_) => true,
            _ => false,
        };
    }

    pub fn is_left_paren(&self) -> bool {
        return match self {
            Self::LeftParen(_) => true,
            _ => false,
        };
    }

    pub fn is_number(&self) -> bool {
        return match self {
            Self::Number(_) => true,
            _ => false,
        };
    }

    pub fn is_float(&self) -> bool {
        return match self {
            Self::Number(v) => v.is_float(),
            _ => false,
        };
    }

    pub fn is_int(&self) -> bool {
        return match self {
            Self::Number(v) => v.is_int(),
            _ => false,
        };
    }

    pub fn is_null(&self) -> bool {
        return match self {
            Self::Text(v) => v.is_null(),
            _ => false,
        };
    }

    pub fn is_right_brace(&self) -> bool {
        return match self {
            Self::RightBrace(_) => true,
            _ => false,
        };
    }

    pub fn is_right_paren(&self) -> bool {
        return match self {
            Self::RightParen(_) => true,
            _ => false,
        };
    }

    pub fn is_string(&self) -> bool {
        return match self {
            Self::Text(v) => v.is_string(),
            _ => false,
        };
    }

    pub fn is_text(&self) -> bool {
        return match self {
            Self::Text(_) => true,
            _ => false,
        };
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Colon(v) => write!(f, "{}", v),
            Self::Comma(v) => write!(f, "{}", v),
            Self::LeftBrace(v) => write!(f, "{}", v),
            Self::LeftParen(v) => write!(f, "{}", v),
            Self::Number(v) => write!(f, "{}", v),
            Self::RightBrace(v) => write!(f, "{}", v),
            Self::RightParen(v) => write!(f, "{}", v),
            Self::Text(v) => write!(f, "{}", v),
        };
    }
}
