mod method_item;

pub use method_item::*;

use crate::{Path, TypeId};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Impl {
    pub module: Path,
    pub for_ty: TypeId,
    pub items: Vec<ImplItem>,
}

impl Impl {
    pub fn refs(&self) -> Box<[TypeId]> {
        let items = self.items.iter().flat_map(|i| i.refs()).collect::<Vec<_>>();
        vec![vec![self.for_ty.clone()], items]
            .concat()
            .into_boxed_slice()
    }
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

impl ImplItem {
    pub fn refs(&self) -> Box<[TypeId]> {
        match self {
            Self::Method(v) => v.refs(),
        }
    }
}

impl std::fmt::Display for ImplItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Method(v) => write!(f, "{}", v),
        }
    }
}
