use crate::Layout;

///
/// ### Pointer Layout
/// pointer sized
///
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct PtrLayout {
    pub ty: PtrLayoutType, //
    pub size: usize,       // the size in bytes the type occupies
    pub align: usize,      // starting point, must be power of two
}

impl PtrLayout {
    pub fn auto(ty: PtrLayoutType) -> Self {
        Self {
            ty,
            size: size_of::<usize>(),
            align: align_of::<usize>(),
        }
    }

    pub fn custom(ty: PtrLayoutType, size: usize, align: usize) -> Self {
        assert!(align.is_power_of_two());
        Self { ty, size, align }
    }
}

impl From<PtrLayout> for Layout {
    fn from(value: PtrLayout) -> Self {
        Self::Ptr(value)
    }
}

impl std::fmt::Display for PtrLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PtrLayoutType {
    Raw,
    Ref,
    Gc,
}

impl std::fmt::Display for PtrLayoutType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Raw => write!(f, "Ptr::Raw"),
            Self::Ref => write!(f, "Ptr::Ref"),
            Self::Gc => write!(f, "Ptr::Gc"),
        }
    }
}
