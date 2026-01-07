use std::{backtrace::Backtrace, rc::Rc};

use crate::{Error, StdError, ZinqError, ZinqErrorCode};

#[derive(Debug, Default, Clone)]
pub struct ErrorBuilder {
    code: Option<ZinqErrorCode>,
    message: Option<String>,
    source: Option<Rc<dyn StdError>>,
    children: Vec<ZinqError>,
}

impl ErrorBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn code(mut self, code: ZinqErrorCode) -> Self {
        self.code = Some(code);
        self
    }

    pub fn message<T: ToString>(mut self, message: T) -> Self {
        self.message = Some(message.to_string());
        self
    }

    pub fn source<Err: StdError>(mut self, source: Err) -> Self {
        self.source = Some(Rc::new(source));
        self
    }

    pub fn child(mut self, child: ZinqError) -> Self {
        self.children.push(child);
        self
    }

    pub fn build(self) -> Error {
        let backtrace = Backtrace::force_capture();

        Error {
            code: self.code.expect("expected code"),
            message: self.message,
            source: self.source,
            backtrace: Some(Rc::new(backtrace)),
            children: self.children,
        }
    }
}

impl<T: StdError> From<T> for ErrorBuilder {
    fn from(value: T) -> Self {
        ErrorBuilder::new().source(value)
    }
}
