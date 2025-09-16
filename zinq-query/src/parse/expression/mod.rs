mod arg;
mod field;
mod object;

pub use arg::*;
pub use field::*;
pub use object::*;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Expression {
    Arg(Arg),
    Field(Field),
    Object(Object),
}

impl Expression {
    pub fn parse(scan: &mut crate::parse::bytes::Scanner<'_>) -> Result<Self, crate::ParseError> {
        return match Object::parse(scan) {
            Err(err) => Err(err),
            Ok(v) => Ok(Self::Object(v)),
        };
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Arg(v) => write!(f, "{}", v),
            Self::Field(v) => write!(f, "{}", v),
            Self::Object(v) => write!(f, "{}", v),
        };
    }
}
