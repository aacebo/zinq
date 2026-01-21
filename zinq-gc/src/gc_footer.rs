use std::any::type_name_of_val;

///
/// ### GcFooter
/// Compact “footer” stored at the *end* of an allocation.
/// Encodes both: (a) what kind of variable-sized object this is, and
/// (b) the dynamic sizing info needed to interpret the payload.
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum GcFooter {
    Fixed,
    Array(GcArrayFooter),
}

impl GcFooter {
    pub fn is_fixed(&self) -> bool {
        match self {
            Self::Fixed => true,
            _ => false,
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            Self::Array(_) => true,
            _ => false,
        }
    }

    pub fn as_array(&self) -> &GcArrayFooter {
        match self {
            Self::Array(v) => v,
            v => panic!(
                "{}",
                format!("expected GcArrayFooter, received {}", type_name_of_val(v))
            ),
        }
    }
}

///
/// ### GcArrayFooter
/// Element payload (array / vector-like).
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GcArrayFooter {
    /// max capacity
    pub cap: u32,

    /// length of data
    pub length: u32,

    /// element stride in bytes
    pub stride: u32,

    /// flags used to describe the data
    pub flags: u8,

    /// reserved for future use
    pub padding: u16,
}

impl From<GcArrayFooter> for GcFooter {
    fn from(value: GcArrayFooter) -> Self {
        Self::Array(value)
    }
}
