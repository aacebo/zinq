#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Path(Vec<String>);

impl Path {}

impl From<&str> for Path {
    fn from(value: &str) -> Self {
        return Self(
            value
                .split("::")
                .filter(|v| *v != "r#mod")
                .map(|v| v.to_string())
                .collect::<Vec<_>>(),
        );
    }
}

impl From<String> for Path {
    fn from(value: String) -> Self {
        return Self(
            value
                .split("::")
                .filter(|v| *v != "r#mod")
                .map(|v| v.to_string())
                .collect::<Vec<_>>(),
        );
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", &self.0.join("::"));
    }
}
