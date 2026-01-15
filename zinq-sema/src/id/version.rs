#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Version {
    V1,
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::V1 => write!(f, "V1"),
        }
    }
}

impl Default for Version {
    fn default() -> Self {
        Self::V1
    }
}
