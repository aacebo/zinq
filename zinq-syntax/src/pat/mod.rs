mod any_pattern;
mod literal_pattern;
mod struct_pattern;
mod type_pattern;

pub use any_pattern::*;
pub use literal_pattern::*;
pub use struct_pattern::*;
pub use type_pattern::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pattern {
    Any(AnyPattern),
    Literal(LiteralPattern),
    Struct(StructPattern),
    Type(TypePattern),
}
