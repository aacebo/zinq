use std::sync::Arc;

use crate::{Code, Error, StdError, ToError};

#[derive(Debug, Default, Clone)]
pub struct Builder(Error);

impl Builder {
    pub fn new() -> Self {
        Self(Error::default())
    }

    pub fn with_code(mut self, code: Code) -> Self {
        self.0.code = code;
        self
    }

    pub fn with_message<T: ToString>(mut self, message: T) -> Self {
        self.0.message = Some(message.to_string());
        self
    }

    pub fn with_source<Err: StdError>(mut self, source: Err) -> Self {
        self.0.source = Some(Arc::new(source));
        self
    }

    pub fn with_child<Err: StdError>(mut self, child: Err) -> Self {
        self.0.children.push(child.to_error());
        self
    }

    pub fn build(self) -> Error {
        self.0
    }
}

impl<T: StdError> From<T> for Builder {
    fn from(value: T) -> Self {
        Builder::new().with_source(value)
    }
}
