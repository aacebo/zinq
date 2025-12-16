use super::Error;

#[derive(Debug, Clone)]
pub struct ListError {
    errors: Vec<Error>,
}

impl ListError {
    pub fn new() -> Self {
        Self { errors: vec![] }
    }

    pub fn len(&self) -> usize {
        self.errors.len()
    }

    pub fn push(&mut self, error: Error) {
        self.errors.push(error);
    }

    pub fn with(&self, error: &Error) -> Self {
        let mut next = self.clone();
        next.errors.push(error.clone());
        next
    }
}

impl<const N: usize> From<[Error; N]> for ListError {
    fn from(value: [Error; N]) -> Self {
        Self {
            errors: value.to_vec(),
        }
    }
}

impl From<&[Error]> for ListError {
    fn from(value: &[Error]) -> Self {
        Self {
            errors: value.to_vec(),
        }
    }
}

impl Into<Error> for ListError {
    fn into(self) -> Error {
        Error::List(self)
    }
}

impl std::fmt::Display for ListError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for error in &self.errors {
            write!(f, "[mx::error] {}", error)?;
        }

        Ok(())
    }
}

impl std::error::Error for ListError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl std::ops::Deref for ListError {
    type Target = [Error];

    fn deref(&self) -> &Self::Target {
        &self.errors
    }
}

impl std::ops::DerefMut for ListError {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.errors
    }
}

impl std::ops::Index<usize> for ListError {
    type Output = Error;

    fn index(&self, index: usize) -> &Self::Output {
        self.errors.index(index)
    }
}

impl std::ops::IndexMut<usize> for ListError {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.errors.index_mut(index)
    }
}

#[cfg(test)]
mod test {
    use crate::error::{ListError, TextError};

    #[test]
    pub fn should_create_error_list() {
        let group = ListError::new().with(&TextError::from("a").into()).with(
            &ListError::from([TextError::from("b").into(), TextError::from("b").into()]).into(),
        );

        debug_assert_eq!(group.len(), 2);
    }
}
