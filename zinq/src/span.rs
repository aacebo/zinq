use std::path::Path;

use crate::Location;

#[derive(Debug, Clone)]
pub struct Span {
    path: Option<Box<Path>>,
    start: Location,
    end: Location,
}

impl Span {
    pub fn path(&self) -> Option<&Path> {
        match &self.path {
            None => None,
            Some(v) => Some(v.as_ref()),
        }
    }

    pub fn start(&self) -> &Location {
        &self.start
    }

    pub fn end(&self) -> &Location {
        &self.end
    }
}
