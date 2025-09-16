mod field;

pub use field::*;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Context<P, T> {
    Field(FieldContext<P, T>),
}
