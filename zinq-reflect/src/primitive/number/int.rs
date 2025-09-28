use crate::TypeOf;

macro_rules! int_type {
    ($($name:ident $type_name:ident $is_type:ident $to_type:ident $type:ty)*) => {
        #[derive(Debug, Copy, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum IntType {
            $($name($type_name),)*
        }

        impl crate::Type {
            $(
                pub fn $is_type(&self) -> bool {
                    return match self {
                        Self::Number(v) => v.$is_type(),
                        _ => false,
                    };
                }

                pub fn $to_type(&self) -> $type_name {
                    return match self {
                        Self::Number(v) => v.$to_type(),
                        _ => panic!("called '{}' on type '{}'", stringify!($to_type), stringify!($name)),
                    };
                }
            )*
        }

        impl crate::NumberType {
            $(
                pub fn $is_type(&self) -> bool {
                    return match self {
                        Self::Int(v) => v.$is_type(),
                        _ => false,
                    };
                }

                pub fn $to_type(&self) -> $type_name {
                    return match self {
                        Self::Int(v) => v.$to_type(),
                        _ => panic!("called '{}' on type '{}'", stringify!($to_type), stringify!($name)),
                    };
                }
            )*
        }

        impl IntType {
            pub fn id(&self) -> crate::TypeId {
                return match self {
                    $(Self::$name(v) => v.id(),)*
                };
            }

            pub fn len(&self) -> usize {
                return match self {
                    $(Self::$name(v) => v.len(),)*
                };
            }

            pub fn is_signed(&self) -> bool {
                return match self {
                    $(Self::$name(v) => v.is_signed(),)*
                };
            }

            $(
                pub fn $is_type(&self) -> bool {
                    return match self {
                        Self::$name(_) => true,
                        _ => false,
                    };
                }

                pub fn $to_type(&self) -> $type_name {
                    return match self {
                        Self::$name(v) => v.clone(),
                        _ => panic!("called '{}' on type '{}'", stringify!($to_type), stringify!($name)),
                    };
                }
            )*

            pub fn assignable_to(&self, ty: crate::Type) -> bool {
                return match self {
                    $(Self::$name(v) => v.assignable_to(ty),)*
                };
            }

            pub fn convertable_to(&self, ty: crate::Type) -> bool {
                return match self {
                    $(Self::$name(v) => v.convertable_to(ty),)*
                };
            }
        }

        impl crate::ToType for IntType {
            fn to_type(&self) -> crate::Type {
                return crate::Type::Number(crate::NumberType::Int(self.clone()));
            }
        }

        impl std::fmt::Display for IntType {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                return match self {
                    $(Self::$name(v) => write!(f, "{}", v),)*
                };
            }
        }

        $(
            #[derive(Debug, Copy, Clone, PartialEq, Default)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct $type_name;

            impl $type_name {
                pub fn id(&self) -> crate::TypeId {
                    return crate::TypeId::from_str(stringify!($type));
                }

                pub fn len(&self) -> usize {
                    panic!("'Type::len()' called on non-array type '{}'", stringify!($name));
                }

                pub fn is_signed(&self) -> bool {
                    return stringify!($name).starts_with("I");
                }

                pub fn assignable_to(&self, ty: crate::Type) -> bool {
                    return self.id() == ty.id();
                }

                pub fn convertable_to(&self, ty: crate::Type) -> bool {
                    return ty.is_int() && self.is_signed() == ty.is_signed();
                }
            }

            impl std::fmt::Display for $type_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    return write!(f, "{}", self.id());
                }
            }

            impl crate::TypeOf for $type {
                fn type_of() -> crate::Type {
                    return crate::Type::Number(crate::NumberType::Int(crate::IntType::$name($type_name)));
                }
            }

            impl crate::ToType for $type {
                fn to_type(&self) -> crate::Type {
                    return crate::Type::Number(crate::NumberType::Int(IntType::$name($type_name)));
                }
            }
        )*
    };
}

macro_rules! int_value {
    ($($name:ident $type_name:ident $is_type:ident $to_type:ident $type:ty)*) => {
        #[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum Int {
            $($name($name),)*
        }

        impl crate::Value {
            $(
                pub fn $is_type(&self) -> bool {
                    return match self {
                        Self::Number(v) => v.$is_type(),
                        _ => false,
                    };
                }

                pub fn $to_type(&self) -> $name {
                    return match self {
                        Self::Number(v) => v.$to_type(),
                        _ => panic!("called '{}' on type '{}'", stringify!($to_type), stringify!($name)),
                    };
                }
            )*
        }

        impl crate::Number {
            $(
                pub fn $is_type(&self) -> bool {
                    return match self {
                        Self::Int(v) => v.$is_type(),
                        _ => false,
                    };
                }

                pub fn $to_type(&self) -> $name {
                    return match self {
                        Self::Int(v) => v.$to_type(),
                        _ => panic!("called '{}' on type '{}'", stringify!($to_type), stringify!($name)),
                    };
                }
            )*
        }

        impl Int {
            pub fn to_type(&self) -> crate::Type {
                return match self {
                    $(Self::$name(v) => v.to_type(),)*
                };
            }

            $(
                pub fn $is_type(&self) -> bool {
                    return match self {
                        Self::$name(_) => true,
                        _ => false,
                    };
                }

                pub fn $to_type(&self) -> $name {
                    return match self {
                        Self::$name(v) => v.clone(),
                        _ => panic!("called '{}' on type '{}'", stringify!($to_type), stringify!($name)),
                    };
                }
            )*
        }

        impl Eq for Int {}

        impl Ord for Int {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                return match self {
                    $(Self::$name(v) => other.$to_type().cmp(v),)*
                };
            }
        }

        impl crate::ToType for Int {
            fn to_type(&self) -> crate::Type {
                return match self {
                    $(Self::$name(v) => v.to_type(),)*
                };
            }
        }

        impl crate::ToValue for Int {
            fn to_value(self) -> crate::Value {
                return match self {
                    $(Self::$name(v) => v.to_value(),)*
                };
            }
        }

        impl std::fmt::Display for Int {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                return match self {
                    $(Self::$name(v) => write!(f, "{}", v),)*
                };
            }
        }

        $(
            #[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct $name($type);

            impl $name {
                pub fn to_type(&self) -> crate::Type {
                    return <$type>::type_of();
                }

                pub fn get(&self) -> $type {
                    return self.0;
                }
            }

            impl Eq for crate::$name {}

            impl Ord for crate::$name {
                fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                    return self.0.cmp(&other.0);
                }
            }

            impl From<$type> for crate::Value {
                fn from(value: $type) -> Self {
                    return Self::Number(crate::Number::Int(crate::Int::$name($name(value))));
                }
            }

            impl Into<$type> for crate::Value {
                fn into(self) -> $type {
                    return self.$to_type().get();
                }
            }

            impl From<$type> for $name {
                fn from(value: $type) -> Self {
                    return Self(value);
                }
            }

            impl Into<$type> for $name {
                fn into(self) -> $type {
                    return self.0;
                }
            }

            impl AsRef<$type> for $name {
                fn as_ref(&self) -> &$type {
                    return &self.0;
                }
            }

            impl AsMut<$type> for $name {
                fn as_mut(&mut self) -> &mut $type {
                    return &mut self.0;
                }
            }

            impl std::ops::Deref for $name {
                type Target = $type;

                fn deref(&self) -> &Self::Target {
                    return &self.0;
                }
            }

            impl std::ops::DerefMut for $name {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    return &mut self.0;
                }
            }

            impl std::fmt::Display for $name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    return write!(f, "{}", self.as_ref());
                }
            }

            impl crate::TypeOf for $name {
                fn type_of() -> crate::Type {
                    return <$type>::type_of();
                }
            }

            impl crate::ToType for $name {
                fn to_type(&self) -> crate::Type {
                    return <$type>::type_of();
                }
            }

            impl crate::ToValue for $name {
                fn to_value(self) -> crate::Value {
                    return crate::Value::Number(crate::Number::Int(Int::$name(self.clone())));
                }
            }

            impl crate::ToValue for $type {
                fn to_value(self) -> crate::Value {
                    return crate::Value::Number(crate::Number::Int(Int::$name($name(self.clone()))));
                }
            }
        )*
    };
}

int_type! {
    I8 I8Type is_i8 to_i8 i8
    I16 I16Type is_i16 to_i16 i16
    I32 I32Type is_i32 to_i32 i32
    I64 I64Type is_i64 to_i64 i64

    U8 U8Type is_u8 to_u8 u8
    U16 U16Type is_u16 to_u16 u16
    U32 U32Type is_u32 to_u32 u32
    U64 U64Type is_u64 to_u64 u64
}

int_value! {
    I8 I8Type is_i8 to_i8 i8
    I16 I16Type is_i16 to_i16 i16
    I32 I32Type is_i32 to_i32 i32
    I64 I64Type is_i64 to_i64 i64

    U8 U8Type is_u8 to_u8 u8
    U16 U16Type is_u16 to_u16 u16
    U32 U32Type is_u32 to_u32 u32
    U64 U64Type is_u64 to_u64 u64
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    pub fn i8() {
        let value = value_of!(125_i8);

        assert!(value.is_int());
        assert!(value.to_type().is_signed());
        assert!(value.is_i8());
        assert_eq!(value.to_i8().get(), 125);
    }

    #[test]
    pub fn i16() {
        let value = value_of!(-15_i16);

        assert!(value.is_int());
        assert!(value.to_type().is_signed());
        assert!(value.is_i16());
        assert_eq!(value.to_i16().get(), -15);
    }

    #[test]
    pub fn i32() {
        let value = value_of!(-15_i32);

        assert!(value.is_int());
        assert!(value.to_type().is_signed());
        assert!(value.is_i32());
        assert_eq!(value.to_i32().get(), -15);
    }

    #[test]
    pub fn i64() {
        let value = value_of!(-15_i64);

        assert!(value.is_int());
        assert!(value.to_type().is_signed());
        assert!(value.is_i64());
        assert_eq!(value.to_i64().get(), -15);
    }

    #[test]
    pub fn u8() {
        let value = value_of!(15_u8);

        assert!(value.is_int());
        assert!(!value.to_type().is_signed());
        assert!(value.is_u8());
        assert_eq!(value.to_u8().get(), 15);
    }

    #[test]
    pub fn u16() {
        let value = value_of!(15_u16);

        assert!(value.is_int());
        assert!(!value.to_type().is_signed());
        assert!(value.is_u16());
        assert_eq!(value.to_u16().get(), 15);
    }

    #[test]
    pub fn u32() {
        let value = value_of!(15_u32);

        assert!(value.is_int());
        assert!(!value.to_type().is_signed());
        assert!(value.is_u32());
        assert_eq!(value.to_u32().get(), 15);
    }

    #[test]
    pub fn u64() {
        let value = value_of!(15_u64);

        assert!(value.is_int());
        assert!(!value.to_type().is_signed());
        assert!(value.is_u64());
        assert_eq!(value.to_u64().get(), 15);
    }
}
