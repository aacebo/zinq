use zinq_reflect::{Reflect, TypeOf};

#[cfg(feature = "macros")]
pub use zinq_error_macros::*;

pub trait ToError {
    fn to_error(&self) -> Error;
}

#[derive(Debug, Clone, Reflect)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Error {
    path: String,
    name: Option<String>,
    code: Option<u16>,
    message: Option<String>,
    children: Vec<Self>,
}

impl Error {
    pub fn new() -> ErrorBuilder {
        return ErrorBuilder::new();
    }

    pub fn path(&self) -> &str {
        return &self.path;
    }

    pub fn name(&self) -> Option<&str> {
        return match &self.name {
            None => None,
            Some(v) => Some(&v),
        };
    }

    pub fn code(&self) -> Option<u16> {
        return self.code;
    }

    pub fn message(&self) -> Option<&str> {
        return match &self.message {
            None => None,
            Some(v) => Some(&v),
        };
    }

    pub fn children(&self) -> &[Self] {
        return &self.children;
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(name) = &self.name {
            write!(f, "{}", name)?;

            if self.message.is_some() {
                write!(f, ": ")?;
            }
        }

        if let Some(message) = &self.message {
            write!(f, "{}", message)?;
        }

        if self.children.is_empty() {
            return Ok(());
        }

        write!(f, " [")?;

        for (i, error) in self.children.iter().enumerate() {
            write!(f, "\n\t{}", error)?;

            if i < self.children.len() - 1 {
                write!(f, ",")?;
            }
        }

        return write!(f, "]");
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        if self.children.len() > 0 {
            let source = self.children.first().unwrap();
            return Some(source);
        }

        return None;
    }
}

///
/// Builder
///
#[derive(Debug, Clone)]
pub struct ErrorBuilder(Error);

impl ErrorBuilder {
    pub fn new() -> Self {
        return Self(Error {
            path: String::new(),
            name: None,
            code: None,
            message: None,
            children: vec![],
        });
    }

    pub fn with_path(&self, path: &str) -> Self {
        let mut next = self.clone();
        next.0.path = path.to_string();
        return next;
    }

    pub fn with_name(&self, name: &str) -> Self {
        let mut next = self.clone();
        next.0.name = Some(name.to_string());
        return next;
    }

    pub fn with_code(&self, code: u16) -> Self {
        let mut next = self.clone();
        next.0.code = Some(code);
        return next;
    }

    pub fn with_message(&self, message: &str) -> Self {
        let mut next = self.clone();
        next.0.message = Some(message.to_string());
        return next;
    }

    pub fn with_child(&self, error: Error) -> Self {
        let mut next = self.clone();
        next.0.children.push(error);
        return next;
    }

    pub fn with_children(&self, errors: &[Error]) -> Self {
        let mut next = self.clone();

        for error in errors {
            next.0.children.push(error.clone());
        }

        return next;
    }

    pub fn build(&self) -> Error {
        return self.0.clone();
    }
}
