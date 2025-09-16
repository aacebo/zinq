/// ## String
///
/// a schema that represents data of String type
pub fn string() -> String {
    return String::new();
}

/// ## String
///
/// a schema that represents data of String type
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct String;

impl String {
    pub fn new() -> Self {
        return Self;
    }
}

impl crate::Schema for String {
    fn name(&self) -> &'static str {
        return "string";
    }
}
