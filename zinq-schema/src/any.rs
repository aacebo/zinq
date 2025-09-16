/// ## Any
///
/// a schema that represents data of any type
pub fn any() -> Any {
    return Any::new();
}

/// ## Any
///
/// a schema that represents data of any type
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Any;

impl Any {
    pub fn new() -> Self {
        return Self;
    }
}

impl crate::Schema for Any {
    fn name(&self) -> &'static str {
        return "any";
    }
}
