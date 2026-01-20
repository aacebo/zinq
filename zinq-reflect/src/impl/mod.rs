mod method_item;

pub use method_item::*;

use crate::{Path, ty::PtrType};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Impl {
    pub module: Path,
    pub for_ty: PtrType,
    pub items: Vec<ImplItem>,
}

impl std::fmt::Display for Impl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "impl {}", &self.for_ty)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ImplItem {
    Method(MethodItem),
}

impl std::fmt::Display for ImplItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Method(v) => write!(f, "{}", v),
        }
    }
}
