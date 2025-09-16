#[macro_export]
macro_rules! value_of {
    [$value:expr] => {
        crate::Reflect::reflect($value)
    };
}

/// ## Reflect
///
/// implemented by all types that
/// can be reflected
pub trait Reflect {
    fn reflect(self) -> crate::Value;
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

        assert!(value.is_ptr());
        assert!(value.is_ptr_of(type_of!(i8)));
        assert_eq!(value.to_ptr().get().to_i8().get(), 1);
    }
}
