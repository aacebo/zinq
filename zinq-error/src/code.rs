#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Code {
    Unknown,
    NotFound,
    BadArguments,
}

impl std::fmt::Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::NotFound => write!(f, "Not Found"),
            Self::BadArguments => write!(f, "Bad Arguments"),
        }
    }
}

impl Default for Code {
    fn default() -> Self {
        Self::Unknown
    }
}
