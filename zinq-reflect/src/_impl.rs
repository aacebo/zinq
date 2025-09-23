#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Impl {
    pub(crate) from: Option<crate::TraitType>,
    pub(crate) methods: Vec<crate::Method>,
}

impl Impl {}
