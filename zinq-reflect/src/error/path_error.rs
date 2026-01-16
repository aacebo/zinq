use std::rc::Rc;

use zinq_error::ZinqError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathError(String);

impl PathError {
    pub fn new(path: &str) -> Self {
        Self(path.to_string())
    }
}

impl Into<ZinqError> for PathError {
    fn into(self) -> ZinqError {
        ZinqError::Std(Rc::new(self))
    }
}

impl std::fmt::Display for PathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{}` is not a valid type path", &self.0)
    }
}

impl std::error::Error for PathError {}
