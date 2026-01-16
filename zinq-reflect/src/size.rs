#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Size {
    Static(usize), // stack
    Dynamic,       // heap
}

impl Size {
    pub fn is_static(&self) -> bool {
        match self {
            Self::Static(_) => true,
            _ => false,
        }
    }

    pub fn is_dynamic(&self) -> bool {
        match self {
            Self::Dynamic => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Static(v) => write!(f, "Size::Static({})", v),
            Self::Dynamic => write!(f, "Size::Dynamic"),
        }
    }
}
