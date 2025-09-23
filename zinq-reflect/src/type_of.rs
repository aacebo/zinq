#[macro_export]
macro_rules! type_of {
    [&$value:expr] => {
        (&$value).to_type()
    };
    [$($type:ty)*] => {
        $(<$type>::type_of())*
    };
    [$value:expr] => {
        (&$value).to_type()
    };
}

pub trait TypeOf {
    fn type_of() -> crate::Type;
    fn to_type(&self) -> crate::Type {
        return Self::type_of();
    }
}

#[cfg(test)]
mod test {
    use crate::TypeOf;

    #[test]
    pub fn from_expr() {
        assert!(type_of!(1_i8).is_i8());
    }

    #[test]
    pub fn from_expr_ref() {
        assert!(type_of!(&1_i8).is_i8());
    }

    #[test]
    pub fn from_type() {
        assert!(type_of!(i8).is_i8());
    }
}
