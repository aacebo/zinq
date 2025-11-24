mod struct_element;

pub use struct_element::*;

pub enum Element {
    Struct(std::sync::Arc<dyn StructElement>),
}
