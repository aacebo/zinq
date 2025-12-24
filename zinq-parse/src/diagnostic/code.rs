use crate::{
    Span,
    diagnostic::{Diagnostic, Severity},
};

///
/// ## Code
/// a predefined type of diagnostic that
/// can be used to emit known code types
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Code {
    pub id: u16,                   // 0
    pub severity: Severity,        // Severity::Error
    pub description: &'static str, // a not found error...
}

impl Code {
    pub fn at(self, span: Span) -> Diagnostic {
        Diagnostic {
            code: self,
            span,
            message: None,
            children: vec![],
        }
    }

    pub fn name(&self) -> String {
        format!("{}{:04}", self.severity.prefix(), self.id)
    }
}
