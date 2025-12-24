mod error;

pub use error::*;

use crate::diagnostic::{Code, Severity};

pub const NOOP: Code = Code {
    id: 0,
    severity: Severity::None,
    name: "Noop",
    description: "no operation",
};
