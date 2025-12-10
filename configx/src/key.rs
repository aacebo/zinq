#[derive(Debug, Clone, PartialEq)]
pub enum Key {
    String(String), // key in an object
    Index(usize),   // index in an array
}

impl From<&str> for Key {
    fn from(value: &str) -> Self {
        return Self::from(value.to_string());
    }
}

impl From<String> for Key {
    fn from(value: String) -> Self {
        match value.parse::<usize>() {
            Err(_) => Self::String(value),
            Ok(v) => Self::Index(v),
        }
    }
}

impl From<usize> for Key {
    fn from(value: usize) -> Self {
        return Self::Index(value);
    }
}

impl From<&usize> for Key {
    fn from(value: &usize) -> Self {
        return Self::Index(*value);
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::String(v) => write!(f, "{}", v),
            Self::Index(v) => write!(f, "{}", v),
        };
    }
}

impl Eq for Key {}

impl PartialEq<&str> for Key {
    fn eq(&self, other: &&str) -> bool {
        return match self {
            Self::String(v) => v == *other,
            _ => false,
        };
    }
}

impl PartialEq<usize> for Key {
    fn eq(&self, other: &usize) -> bool {
        return match self {
            Self::Index(v) => v == other,
            _ => false,
        };
    }
}

impl std::ops::Deref for Key {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        return match self {
            Self::String(v) => &v,
            _ => panic!("expected string value"),
        };
    }
}
