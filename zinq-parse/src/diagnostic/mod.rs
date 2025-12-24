mod code;
mod severity;

pub use code::*;
pub use severity::*;

use crate::Span;

///
/// ## Diagnostic
/// some error/warning/info about a
/// span of source code flagged by the
/// compiler
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Diagnostic {
    pub code: Code,
    pub span: Span,
    pub message: Option<String>,
    pub children: Vec<Diagnostic>,
}

impl Diagnostic {
    pub fn message(mut self, message: &str) -> Self {
        self.message = Some(message.to_string());
        self
    }

    pub fn add(mut self, child: Diagnostic) -> Self {
        self.children.push(child);
        self
    }
}

impl std::fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{}]: ", &self.code.severity, &self.code.name())?;

        if let Some(message) = &self.message {
            write!(f, "{}", message)?;
        } else {
            write!(f, "an unknown error occurred")?;
        }

        write!(f, "\n  --> ")?;

        if let Some(file) = self.span.file() {
            write!(f, "{}:", file.path())?;
        }

        writeln!(
            f,
            "{}:{}",
            self.span.start().line(),
            self.span.start().column()
        )?;

        writeln!(f, "   |\t{}", &self.span)?;
        Ok(())
    }
}
