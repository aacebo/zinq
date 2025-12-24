mod code;
mod codes;
mod severity;

pub use code::*;
pub use codes::*;
pub use severity::*;

use std::{backtrace::Backtrace, rc::Rc};

use crate::Span;

///
/// ## Diagnostic
/// some error/warning/info about a
/// span of source code flagged by the
/// compiler
/// ### Examples
/// #### Print Error
/// ```bash
/// ./src/main.zq
///   --> error [E0005](3,5): Unknown Identifier "print"
///    |    print(...);
/// ```
/// #### Print Warning
/// ./src/main.zq
///   --> warn [W1024](15,4): Unused variable "args"
///    |    var args = (...);
///
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub code: Code,
    pub span: Span,
    pub message: Option<String>,
    pub backtrace: Option<Rc<Backtrace>>,
    pub children: Vec<Diagnostic>,
}

impl Diagnostic {
    pub fn new(span: Span) -> Self {
        Diagnostic {
            code: NOOP,
            span,
            message: None,
            backtrace: None,
            children: vec![],
        }
    }

    pub fn message(mut self, message: &str) -> Self {
        self.message = Some(message.to_string());
        self
    }

    pub fn add(mut self, children: &[Diagnostic]) -> Self {
        self.children.extend_from_slice(children);
        self
    }

    pub fn max_severity(&self) -> &Severity {
        let mut max = &self.code.severity;

        for child in &self.children {
            let cmax = child.max_severity();

            if cmax > max {
                max = cmax
            }
        }

        max
    }
}

impl std::fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.code == NOOP {
            return Ok(());
        }

        write!(f, "{} [{}]: ", &self.code.severity, &self.code.alias())?;

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
            "({},{})",
            self.span.start().line(),
            self.span.start().column()
        )?;

        writeln!(f, "   |\t{}", &self.span)?;
        Ok(())
    }
}
