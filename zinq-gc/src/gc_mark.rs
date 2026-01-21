use zinq_error::{Error, ZinqError};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GcMark {
    /// unreachable/garbage
    White,
    /// reachable but not fully scanned
    Gray,
    /// reachable and fully scanned
    Black,
}

impl GcMark {
    pub fn is_white(&self) -> bool {
        match self {
            Self::White => true,
            _ => false,
        }
    }

    pub fn is_gray(&self) -> bool {
        match self {
            Self::Gray => true,
            _ => false,
        }
    }

    pub fn is_black(&self) -> bool {
        match self {
            Self::Black => true,
            _ => false,
        }
    }
}

impl TryFrom<u8> for GcMark {
    type Error = ZinqError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::White),
            1 => Ok(Self::Gray),
            2 => Ok(Self::Black),
            _ => Err(Error::from_str("invalid GcMark value").build().into()),
        }
    }
}

impl std::fmt::Display for GcMark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::White => write!(f, "GcMark::White"),
            Self::Gray => write!(f, "GcMark::Gray"),
            Self::Black => write!(f, "GcMark::Black"),
        }
    }
}
