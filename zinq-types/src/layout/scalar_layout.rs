use crate::Layout;

///
/// ### Scalar Layout
/// fixed size
///
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ScalarLayout {
    pub size: usize,  // the size in bytes the type occupies
    pub align: usize, // starting point, must be power of two
}

impl ScalarLayout {
    pub fn new(size: usize, align: usize) -> Self {
        assert!(align.is_power_of_two());
        Self { size, align }
    }
}

impl From<ScalarLayout> for Layout {
    fn from(value: ScalarLayout) -> Self {
        Self::Scalar(value)
    }
}

impl std::fmt::Display for ScalarLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}
