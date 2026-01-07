mod error;

pub use error::*;

use crate::{
    Span,
    diagnostic::{Builder, Code, Diagnostic, Severity},
};

pub const NOOP: Code = Code {
    id: 0,
    severity: Severity::None,
    name: "Noop",
    description: "no operation",
};

impl Diagnostic {
    pub fn noop(span: Span) -> Builder {
        Self::new(span).code(NOOP)
    }
}
