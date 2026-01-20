use crate::Layout;

///
/// ### Opaque Layout
/// not known at compile time
///
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct OpaqueLayout {
    pub align: usize, // starting point, must be power of two
}

impl OpaqueLayout {
    pub fn new(align: usize) -> Self {
        assert!(align.is_power_of_two());
        Self { align }
    }
}

impl From<OpaqueLayout> for Layout {
    fn from(value: OpaqueLayout) -> Self {
        Self::Opaque(value)
    }
}

impl std::fmt::Display for OpaqueLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}
