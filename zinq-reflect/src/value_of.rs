#[macro_export]
macro_rules! value_of {
    [$value:expr] => {
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

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    pub fn from_expr() {
        let value = value_of!(1_i8);

        assert!(value.is_i8());
        assert_eq!(value.to_i8().get(), 1);
    }

    #[test]
    pub fn from_expr_ref() {
        let value = value_of!(&1_i8);

        assert!(value.is_ref());
        assert!(value.is_ref_of(type_of!(i8)));
        assert_eq!(value.to_ptr().get().to_i8().get(), 1);
        assert_eq!(value.to_type().id(), "&i8");
    }
}
