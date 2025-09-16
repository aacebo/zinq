mod parse;
mod query;

pub use parse::*;
pub use query::*;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Error {
    Parse(ParseError),
    Query(QueryError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Parse(v) => write!(f, "{}", v),
            Self::Query(v) => write!(f, "{}", v),
        };
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        return match self {
            Self::Parse(v) => v.source(),
            Self::Query(v) => v.source(),
        };
    }
}

impl From<ParseError> for Error {
    fn from(value: ParseError) -> Self {
        return Self::Parse(value);
    }
}

impl From<&ParseError> for Error {
    fn from(value: &ParseError) -> Self {
        return Self::Parse(value.clone());
    }
}

impl From<QueryError> for Error {
    fn from(value: QueryError) -> Self {
        return Self::Query(value);
    }
}

impl From<&QueryError> for Error {
    fn from(value: &QueryError) -> Self {
        return Self::Query(value.clone());
    }
}
