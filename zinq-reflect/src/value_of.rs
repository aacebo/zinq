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

pub trait Object: std::any::Any + std::fmt::Debug + crate::ToType + crate::ToValue {
    fn field(&self, name: &crate::FieldName) -> crate::Value;
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
