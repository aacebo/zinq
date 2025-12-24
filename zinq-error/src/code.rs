pub const UNKNOWN: ErrorCode = ErrorCode {
    id: 0,
    name: "Unknown",
    description: "an error that doesn't match a predefined error type",
};

pub const NOT_FOUND: ErrorCode = ErrorCode {
    id: 1,
    name: "NotFound",
    description: "some resource could not be found",
};

pub const BAD_ARGUMENTS: ErrorCode = ErrorCode {
    id: 2,
    name: "BadArguments",
    description: "invalid input provided",
};

///
/// ## ErrorCode
/// an error type definition
/// ### Examples
/// `NotFound`, `BadArguments`, etc...
///
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ErrorCode {
    pub id: u16,
    pub name: &'static str,
    pub description: &'static str,
}
