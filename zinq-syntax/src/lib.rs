mod bounds;
pub mod expr;
pub mod fields;
mod generics;
pub mod members;
pub mod meta;
pub mod param;
pub mod pat;
mod path;
pub mod spread;
pub mod stmt;
pub mod ty;
mod use_path;
mod variant;
mod visibility;

use std::hash::Hasher;

pub use bounds::*;
pub use generics::*;
pub use path::*;
pub use use_path::*;
pub use variant::*;
pub use visibility::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(u32);

impl std::ops::Deref for NodeId {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

pub trait Node: std::hash::Hash {
    fn name(&self) -> &str;

    fn id(&self) -> NodeId {
        let mut hasher = std::hash::DefaultHasher::new();
        self.hash(&mut hasher);
        NodeId(hasher.finish() as u32)
    }

    #[allow(unused)]
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        unimplemented!()
    }
}

pub trait Visitor:
    ty::TypeVisitor + stmt::StmtVisitor + expr::ExprVisitor + pat::PatternVisitor + param::ParamVisitor
{
}
