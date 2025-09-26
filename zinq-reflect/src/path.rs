#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Path(Vec<String>);

impl Path {
    pub fn len(&self) -> usize {
        return self.0.len();
    }

    pub fn name(&self) -> &str {
        return &self.0.last().unwrap();
    }

    pub fn iter(&self) -> std::slice::Iter<'_, String> {
        return self.0.iter();
    }

    pub fn add(mut self, part: &str) -> Self {
        self.0.push(part.to_string());
        return self;
    }
}

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
