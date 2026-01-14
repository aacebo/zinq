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

pub use bounds::*;
pub use generics::*;
pub use path::*;
pub use use_path::*;
pub use variant::*;
pub use visibility::*;

pub trait Syntax: std::hash::Hash {
    fn name(&self) -> &str;

    #[allow(unused)]
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        unimplemented!()
    }
}

pub trait Visitor:
    ty::TypeVisitor + stmt::StmtVisitor + expr::ExprVisitor + pat::PatternVisitor + param::ParamVisitor
{
}
