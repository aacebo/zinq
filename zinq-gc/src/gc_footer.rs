///
/// ### GcFooter
/// Compact “footer” stored at the *end* of an allocation.
/// Encodes both: (a) what kind of variable-sized object this is, and
/// (b) the dynamic sizing info needed to interpret the payload.
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum GcFooter {
    Fixed(GcFixedFooter),
    Array(GcArrayFooter),
}

impl GcFooter {
    pub fn is_fixed(&self) -> bool {
        match self {
            Self::Fixed(_) => true,
            _ => false,
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            Self::Array(_) => true,
            _ => false,
        }
    }
}

///
/// ### GcFixedFooter
/// Fixed-size object (non-variable payload).
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GcFixedFooter {
    /// header + payload + footer
    size: u32,
}

///
/// ### GcArrayFooter
/// Element payload (array / vector-like).
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GcArrayFooter {
    /// header + payload + footer
    size: u32,

    /// max capacity
    cap: u32,

    /// length of data
    length: u32,

    /// element stride in bytes
    stride: u32,

    /// flags used to describe the data
    flags: u8,

    /// reserved for future use
    padding: u16,
}
