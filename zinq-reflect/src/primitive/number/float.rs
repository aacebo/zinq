use crate::TypeOf;

macro_rules! float_type {
    ($($name:ident $type_name:ident $is_type:ident $to_type:ident $type:ty)*) => {
        #[derive(Debug, Copy, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum FloatType {
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
                        Self::Float(v) => v.$is_type(),
                        _ => false,
                    };
                }

                pub fn $to_type(&self) -> $type_name {
                    return match self {
                        Self::Float(v) => v.$to_type(),
                        _ => panic!("called '{}' on type '{}'", stringify!($to_type), stringify!($name)),
                    };
                }
            )*
        }

        impl FloatType {
            pub fn id(&self) -> crate::TypeId {
                return match self {
                    $(Self::$name(v) => v.id(),)*
                };
            }

            pub fn to_type(&self) -> crate::Type {
                return crate::Type::Number(crate::NumberType::Float(self.clone()));
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

        impl std::fmt::Display for FloatType {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                return match self {
                    $(Self::$name(v) => write!(f, "{}", v),)*
                };
            }
        }

        impl From<std::any::TypeId> for FloatType {
            fn from(value: std::any::TypeId) -> Self {
                return Self::from(&value);
            }
        }

        impl From<&std::any::TypeId> for FloatType {
            fn from(value: &std::any::TypeId) -> Self {
                if value == &std::any::TypeId::of::<f32>() {
                    return Self::F32(F32Type::default());
                } else if value == &std::any::TypeId::of::<f64>() {
                    return Self::F64(F64Type::default());
                }

                panic!("invalid type");
            }
        }

        $(
            #[derive(Debug, Copy, Clone, PartialEq, Default)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct $type_name;

            impl $type_name {
                pub fn id(&self) -> crate::TypeId {
                    return crate::TypeId::from_str(&stringify!($type));
                }

                pub fn assignable_to(&self, ty: crate::Type) -> bool {
                    return self.id() == ty.id();
                }

                pub fn convertable_to(&self, ty: crate::Type) -> bool {
                    return ty.is_float();
                }
            }

            impl std::fmt::Display for $type_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    return write!(f, "{}", self.id());
                }
            }

            impl crate::TypeOf for $type {
                fn type_of() -> crate::Type {
                    return crate::Type::Number(crate::NumberType::Float(crate::FloatType::$name($type_name)));
                }
            }
        )*
    };
}

macro_rules! float_value {
    ($($name:ident $type_name:ident $is_type:ident $to_type:ident $type:ty)*) => {
        #[derive(Debug, Copy, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum Float {
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
                        Self::Float(v) => v.$is_type(),
                        _ => false,
                    };
                }

                pub fn $to_type(&self) -> $name {
                    return match self {
                        Self::Float(v) => v.$to_type(),
                        _ => panic!("called '{}' on type '{}'", stringify!($to_type), stringify!($name)),
                    };
                }
            )*
        }

        impl Float {
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

        impl crate::ToValue for Float {
            fn to_value(self) -> crate::Value {
                return match self {
                    $(Self::$name(v) => v.to_value(),)*
                };
            }
        }

        impl std::fmt::Display for Float {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                return match self {
                    $(Self::$name(v) => write!(f, "{}", v),)*
                };
            }
        }

        $(
            #[derive(Debug, Copy, Clone, PartialEq, Default)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct $name($type);

            impl $name {
                pub fn get(&self) -> $type {
                    return self.0;
                }
            }

            impl From<$type> for crate::Value {
                fn from(value: $type) -> Self {
                    return Self::Number(crate::Number::Float(crate::Float::$name($name(value))));
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

            impl crate::ToValue for $name {
                fn to_value(self) -> crate::Value {
                    return crate::Value::Number(crate::Number::Float(Float::$name(self.clone())));
                }
            }

            impl crate::ToValue for $type {
                fn to_value(self) -> crate::Value {
                    return crate::Value::Number(crate::Number::Float(Float::$name($name(self.clone()))));
                }
            }
        )*
    };
}

float_type! {
    F32 F32Type is_f32 to_f32 f32
    F64 F64Type is_f64 to_f64 f64
}

float_value! {
    F32 F32Type is_f32 to_f32 f32
    F64 F64Type is_f64 to_f64 f64
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    pub fn f32() {
        let value = value_of!(300.26_f32);

        assert!(value.is_float());
        assert!(value.is_f32());
        assert_eq!(value.to_f32().get(), 300.26);
    }

    #[test]
    pub fn f64() {
        let value = value_of!(350.26_f64);

        assert!(value.is_float());
        assert!(value.is_f64());
        assert_eq!(value.to_f64().get(), 350.26);
    }
}
