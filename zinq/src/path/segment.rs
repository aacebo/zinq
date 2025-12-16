#[derive(Debug, Clone, PartialEq)]
pub enum Segment {
    Key(String),  // key in an object
    Index(usize), // index in an array
}

impl From<&str> for Segment {
    fn from(value: &str) -> Self {
        return Self::from(value.to_string());
    }
}

impl From<String> for Segment {
    fn from(value: String) -> Self {
        match value.parse::<usize>() {
            Err(_) => Self::Key(value),
            Ok(v) => Self::Index(v),
        }
    }
}

impl From<usize> for Segment {
    fn from(value: usize) -> Self {
        return Self::Index(value);
    }
}

impl From<&usize> for Segment {
    fn from(value: &usize) -> Self {
        return Self::Index(*value);
    }
}

impl std::fmt::Display for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Key(v) => write!(f, "{}", v),
            Self::Index(v) => write!(f, "{}", v),
        };
    }
}

impl Eq for Segment {}

impl PartialEq<&str> for Segment {
    fn eq(&self, other: &&str) -> bool {
        return match self {
            Self::Key(v) => v == *other,
            _ => false,
        };
    }
}

impl PartialEq<usize> for Segment {
    fn eq(&self, other: &usize) -> bool {
        return match self {
            Self::Index(v) => v == other,
            _ => false,
        };
    }
}

impl std::ops::Deref for Segment {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        return match self {
            Self::Key(v) => &v,
            _ => panic!("expected string value"),
        };
    }
}
