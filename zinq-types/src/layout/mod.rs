mod opaque_layout;
mod ptr_layout;
mod scalar_layout;

pub use opaque_layout::*;
pub use ptr_layout::*;
pub use scalar_layout::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Layout {
    Empty,
    Opaque(OpaqueLayout),
    Ptr(PtrLayout),
    Scalar(ScalarLayout),
}

impl std::fmt::Display for Layout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "Empty"),
            Self::Opaque(v) => write!(f, "{}", v),
            Self::Ptr(v) => write!(f, "{}", v),
            Self::Scalar(v) => write!(f, "{}", v),
        }
    }
}
