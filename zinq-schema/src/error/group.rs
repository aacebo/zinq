#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct GroupError(Vec<super::Error>);

impl GroupError {
    pub fn new() -> Self {
        return Self(vec![]);
    }

    pub fn len(&self) -> usize {
        return self.0.len();
    }

    pub fn add(&mut self, err: super::Error) -> &mut Self {
        self.0.push(err);
        return self;
    }
}
impl std::fmt::Display for GroupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        for (i, error) in self.0.iter().enumerate() {
            write!(f, "\n\t{}", error)?;

            if i < self.len() - 1 {
                write!(f, ",")?;
            }
        }

        return write!(f, "]");
    }
}

impl std::error::Error for GroupError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        return Some(self);
    }
}
