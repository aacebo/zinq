impl<T: crate::TypeOf> crate::TypeOf for Vec<T> {
    fn type_of() -> crate::Type {
        return crate::Type::Void;
    }
}
