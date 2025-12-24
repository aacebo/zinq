use crate::diagnostic::{Code, Severity};

pub const INTERNAL_ERROR: Code = Code {
    id: 0,
    severity: Severity::Error,
    name: "Internal",
    description: "an unknown internal error was encountered.",
};

pub const TOKEN_NOT_FOUND_ERROR: Code = Code {
    id: 1,
    severity: Severity::Error,
    name: "TokenNotFound",
    description: "token not found.",
};
