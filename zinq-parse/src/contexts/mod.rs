mod enum_context;
mod struct_context;

pub use enum_context::*;
pub use struct_context::*;

use syn::parse::Parse;

#[derive(Debug, Clone)]
pub enum Context {
    Enum(EnumContext),
    Struct(StructContext),
}

impl Context {
    pub fn input<T: Parse>(&self) -> Option<T> {
        return match self {
            Self::Enum(v) => v.input(),
            Self::Struct(v) => v.input(),
        };
    }

    pub fn item(&self) -> syn::Item {
        return match self {
            Self::Enum(v) => syn::Item::Enum(v.target.clone()),
            Self::Struct(v) => syn::Item::Struct(v.target.clone()),
        };
    }
}
