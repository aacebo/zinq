pub const UNKNOWN: ZinqErrorCode = ZinqErrorCode {
    id: 0,
    name: "Unknown",
    description: "an error that doesn't match a predefined error type",
};

pub const NOT_FOUND: ZinqErrorCode = ZinqErrorCode {
    id: 1,
    name: "NotFound",
    description: "some resource could not be found",
};

pub const BAD_ARGUMENTS: ZinqErrorCode = ZinqErrorCode {
    id: 2,
    name: "BadArguments",
    description: "invalid input provided",
};

///
/// ## ZinqErrorCode
/// an error type definition
/// ### Examples
/// `NotFound`, `BadArguments`, etc...
///
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ZinqErrorCode {
    pub id: u16,
    pub name: &'static str,
    pub description: &'static str,
}

impl Default for ZinqErrorCode {
    fn default() -> Self {
        UNKNOWN
    }
}
