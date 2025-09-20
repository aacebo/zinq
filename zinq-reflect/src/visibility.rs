#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Visibility {
    Public(Public),
    Private,
}

impl Default for Visibility {
    fn default() -> Self {
        return Visibility::Private;
    }
}

impl std::fmt::Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Public(v) => write!(f, "{}", v),
            Self::Private => Ok(()),
        };
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Public {
    Full,
    Type,
    Super,
    Crate,
    Mod(String),
}

impl std::fmt::Display for Public {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Full => write!(f, "pub"),
            Self::Type => write!(f, "pub(self)"),
            Self::Super => write!(f, "pub(super)"),
            Self::Crate => write!(f, "pub(crate)"),
            Self::Mod(path) => write!(f, "pub(in {})", path),
        };
    }
}
