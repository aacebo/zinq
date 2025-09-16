mod float;
mod int;

pub use float::*;
pub use int::*;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Number {
    Float(Float),
    Int(Int),
}

impl Number {
    pub fn parse(scan: &mut crate::parse::Scanner<'_>) -> Result<super::Token, crate::ParseError> {
        while !scan.is_eof() && scan.peek().unwrap_or(0).is_ascii_digit() {
            scan.next();
        }

        if scan.peek().unwrap_or(0) == b'.' {
            scan.next();

            while !scan.is_eof() && scan.peek().unwrap_or(0).is_ascii_digit() {
                scan.next();
            }

            return match scan.commit().parse::<f64>() {
                Err(err) => {
                    Err(crate::ParseError::new(err.to_string().as_str()).position(scan.position()))
                }
                Ok(value) => Ok(super::Token::Number(Self::Float(Float {
                    position: scan.position(),
                    value,
                }))),
            };
        }

        return match scan.commit().parse::<i64>() {
            Err(err) => {
                Err(crate::ParseError::new(err.to_string().as_str()).position(scan.position()))
            }
            Ok(value) => Ok(super::Token::Number(Self::Int(Int {
                position: scan.position(),
                value,
            }))),
        };
    }
}

impl Number {
    pub fn is_float(&self) -> bool {
        return match self {
            Self::Float(_) => true,
            _ => false,
        };
    }

    pub fn is_int(&self) -> bool {
        return match self {
            Self::Int(_) => true,
            _ => false,
        };
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Float(v) => write!(f, "{}", v),
            Self::Int(v) => write!(f, "{}", v),
        };
    }
}

impl PartialEq<super::Token> for Number {
    fn eq(&self, other: &super::Token) -> bool {
        return match other {
            super::Token::Number(v) => v.eq(self),
            _ => false,
        };
    }
}

impl Into<super::Token> for Number {
    fn into(self) -> super::Token {
        return super::Token::Number(self.clone());
    }
}
