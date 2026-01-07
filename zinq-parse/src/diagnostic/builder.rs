use std::{backtrace::Backtrace, rc::Rc};

use crate::{
    Diagnostic, Span,
    diagnostic::{Code, Severity},
};

#[derive(Debug, Default, Clone)]
pub struct Builder {
    span: Span,
    code: Option<Code>,
    message: Option<String>,
    backtrace: Option<Rc<Backtrace>>,
    children: Vec<Diagnostic>,
}

impl Builder {
    pub fn new(span: Span) -> Self {
        Self {
            span,
            ..Self::default()
        }
    }

    pub fn code(mut self, code: Code) -> Self {
        self.code = Some(code);

        if code.severity == Severity::Error {
            self.backtrace = Some(Rc::new(Backtrace::force_capture()));
        }

        self
    }

    pub fn message(mut self, message: &str) -> Self {
        self.message = Some(message.to_string());
        self
    }

    pub fn child(mut self, child: Diagnostic) -> Self {
        self.children.push(child);
        self
    }

    pub fn children(mut self, children: &[Diagnostic]) -> Self {
        self.children.extend_from_slice(children);
        self
    }

    pub fn build(self) -> Diagnostic {
        Diagnostic {
            code: self.code.expect("expected code"),
            span: self.span,
            message: self.message,
            backtrace: self.backtrace,
            children: self.children,
        }
    }
}
