use std::rc::Rc;

use crate::{Error, ErrorCode, StdError};

#[derive(Debug, Default, Clone)]
pub struct ErrorBuilder(Error);

impl ErrorBuilder {
    pub fn new() -> Self {
        Self(Error::default())
    }

    pub fn code(mut self, code: ErrorCode) -> Self {
        self.0.code = code;
        self
    }

    pub fn message<T: ToString>(mut self, message: T) -> Self {
        self.0.message = Some(message.to_string());
        self
    }

    pub fn source<Err: StdError>(mut self, source: Err) -> Self {
        self.0.source = Some(Rc::new(source));
        self
    }

    pub fn child(mut self, child: Error) -> Self {
        self.0.children.push(Rc::new(child));
        self
    }

    pub fn build(self) -> Error {
        self.0
    }
}

impl<T: StdError> From<T> for ErrorBuilder {
    fn from(value: T) -> Self {
        ErrorBuilder::new().source(value)
    }
}
