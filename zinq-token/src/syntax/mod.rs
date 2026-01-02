pub mod fields;
pub mod ty;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Syntax {
    Type(ty::Type),
}
