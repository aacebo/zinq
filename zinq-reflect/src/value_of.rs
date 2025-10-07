#[macro_export]
macro_rules! value_of {
    [$value:expr] => {
        $crate::ToValue::to_value($value)
    };
    [$value:ident] => {
        $crate::ToValue::to_value($value)
    };
    [$value:literal] => {
        $crate::ToValue::to_value($value)
    };
}

/// ## ToValue
///
/// implemented by types that
/// can reflect their value
pub trait ToValue {
    fn to_value(self) -> crate::Value;
}

/// ## AsValue
///
/// implemented by types that
/// can reflect their value
pub trait AsValue {
    fn as_value(&self) -> Box<crate::Value>;
}

impl<T: Clone + ToValue> AsValue for T {
    fn as_value(&self) -> Box<crate::Value> {
        return Box::new(self.clone().to_value());
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    pub fn from_expr() {
        let value = value_of!(1_i8);

        assert!(value.is_i8());
        assert_eq!(value.to_i8(), 1);
    }

    #[test]
    pub fn from_expr_ref() {
        let value = value_of!(&1_i8);

        assert!(value.is_ref());
        assert_eq!(value.to_ref().to_i8(), 1);
        assert_eq!(value.to_type().id(), "&i8");
    }
}
