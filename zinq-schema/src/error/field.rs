use super::GroupError;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FieldError {
    pub name: String,
    pub message: Option<String>,
    pub errors: Option<GroupError>,
}

impl FieldError {
    pub fn new(name: &str) -> Self {
        return Self {
            name: name.to_string(),
            message: None,
            errors: None,
        };
    }

    pub fn len(&self) -> usize {
        return match &self.errors {
            None => 0,
            Some(errors) => errors.len(),
        };
    }

    pub fn message(self, message: &str) -> Self {
        let mut next = self.clone();
        next.message = Some(message.to_string());
        return next;
    }

    pub fn error(self, err: super::Error) -> Self {
        let mut next = self.clone();

        return match &mut next.errors {
            None => {
                next.errors = Some(GroupError::new());
                next
            }
            Some(errors) => {
                errors.add(err);
                next
            }
        };
    }
}
impl std::fmt::Display for FieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FieldError {{")?;
        write!(f, "\n\tname: \"{}\"", self.name)?;

        if let Some(v) = &self.message {
            write!(f, ",\n\tmessage: \"{}\"", v)?;
        }

        if let Some(v) = &self.errors {
            write!(f, "{}", v)?;
        }

        return write!(f, "\n}}\n");
    }
}

impl std::error::Error for FieldError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        return Some(self);
    }
}
