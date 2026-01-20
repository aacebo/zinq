#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Size {
    Known(usize),
    Opache,
}

impl Size {
    pub fn is_known(&self) -> bool {
        match self {
            Self::Known(_) => true,
            _ => false,
        }
    }

    pub fn is_opache(&self) -> bool {
        match self {
            Self::Opache => true,
            _ => false,
        }
    }
}

impl Default for Size {
    fn default() -> Self {
        Self::Opache
    }
}

impl std::fmt::Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Known(v) => write!(f, "Size::Known({})", v),
            Self::Opache => write!(f, "Size::Opache"),
        }
    }
}
