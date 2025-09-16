use std::fmt;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct Array(Vec<super::Value>);

impl Array {
    pub fn new() -> Self {
        return Self(vec![]);
    }

    pub fn len(&self) -> usize {
        return self.0.len();
    }

    pub fn push(&mut self, value: super::Value) {
        self.0.push(value);
    }

    pub fn pop(&mut self) -> Option<super::Value> {
        return self.0.pop();
    }

    pub fn append(&mut self, items: &mut Vec<super::Value>) {
        self.0.append(items);
    }
}

impl From<Vec<super::Value>> for Array {
    fn from(value: Vec<super::Value>) -> Self {
        return Self(value);
    }
}

impl From<&[super::Value]> for Array {
    fn from(value: &[super::Value]) -> Self {
        return Self(value.to_vec());
    }
}

impl Into<Vec<super::Value>> for Array {
    fn into(self) -> Vec<super::Value> {
        return self.0;
    }
}

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;

        for (i, value) in self.0.iter().enumerate() {
            write!(f, "{}", value)?;

            if i < self.0.len() - 1 {
                write!(f, ",")?;
            }
        }

        return write!(f, "]");
    }
}

impl IntoIterator for Array {
    type IntoIter = std::vec::IntoIter<super::Value>;
    type Item = super::Value;

    fn into_iter(self) -> Self::IntoIter {
        return self.0.into_iter();
    }
}

impl<'a> IntoIterator for &'a Array {
    type IntoIter = std::slice::Iter<'a, super::Value>;
    type Item = &'a super::Value;

    fn into_iter(self) -> Self::IntoIter {
        return self.0.iter();
    }
}
