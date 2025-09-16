use std::fmt;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Number {
    Int(i64),
    Float(f64),
}

impl Number {
    pub fn is_i64(&self) -> bool {
        return match self {
            Self::Int(_) => true,
            _ => false,
        };
    }

    pub fn is_f64(&self) -> bool {
        return match self {
            Self::Float(_) => true,
            _ => false,
        };
    }

    pub fn as_i64(&self) -> i64 {
        return match self {
            Self::Int(v) => *v,
            Self::Float(v) => *v as i64,
        };
    }

    pub fn as_f64(&self) -> f64 {
        return match self {
            Self::Int(v) => *v as f64,
            Self::Float(v) => *v,
        };
    }
}

impl From<i64> for Number {
    fn from(value: i64) -> Self {
        return Self::Int(value);
    }
}

impl From<&i64> for Number {
    fn from(value: &i64) -> Self {
        return Self::Int(*value);
    }
}

impl Into<i64> for Number {
    fn into(self) -> i64 {
        return self.as_i64();
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        return Self::Float(value);
    }
}

impl From<&f64> for Number {
    fn from(value: &f64) -> Self {
        return Self::Float(*value);
    }
}

impl Into<f64> for Number {
    fn into(self) -> f64 {
        return self.as_f64();
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            Self::Int(v) => write!(f, "{}", v),
            Self::Float(v) => write!(f, "{}", v),
        };
    }
}
