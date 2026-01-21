#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ArenaId(u32);

impl ArenaId {
    pub fn as_u32(&self) -> &u32 {
        &self.0
    }

    pub fn to_u32(&self) -> u32 {
        self.0
    }

    pub fn to_usize(&self) -> usize {
        self.0 as usize
    }
}

impl std::ops::Deref for ArenaId {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for ArenaId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}
