use crate::ToType;

macro_rules! tuple_type {
    ($($name:ident $type_name:ident $to_type:ident $len:literal)*) => {
        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum TupleType {
            $($name($type_name),)*
        }

        impl crate::Type {
            $(
                pub fn $to_type(&self) -> $type_name {
                    return match self {
                        Self::Tuple(v) => v.$to_type(),
                        v => panic!("called '{}' on type '{}'", stringify!($to_type), v.id()),
                    };
                }
            )*
        }

        impl TupleType {
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

            pub fn get(&self) -> &[Box<crate::Type>] {
                return match self {
                    $(Self::$name(v) => v.get_ref(),)*
                };
            }

            $(
                pub fn $to_type(&self) -> $type_name {
                    return match self {
                        Self::$name(v) => v.clone(),
                        v => panic!("called '{}' on type '{}'", stringify!($to_type), v.to_type()),
                    };
                }
            )*
        }

        impl crate::ToType for TupleType {
            fn to_type(&self) -> crate::Type {
                return crate::Type::Tuple(self.clone());
            }
        }

        impl std::ops::Index<usize> for TupleType {
            type Output = crate::Type;

            fn index(&self, index: usize) -> &Self::Output {
                return match self {
                    $(Self::$name(v) => v.index(index),)*
                };
            }
        }

        impl std::ops::IndexMut<usize> for TupleType {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                return match self {
                    $(Self::$name(v) => v.index_mut(index),)*
                };
            }
        }

        impl std::fmt::Display for TupleType {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                return match self {
                    $(Self::$name(v) => write!(f, "{}", v),)*
                };
            }
        }

        $(
            #[derive(Debug, Clone, PartialEq)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct $type_name([Box<crate::Type>; $len]);

            impl $type_name {
                pub fn new(types: [Box<crate::Type>; $len]) -> Self {
                    return Self(types);
                }

                pub fn id(&self) -> crate::TypeId {
                    let mut value = String::from("(");

                    for (i, ty) in self.0.iter().enumerate() {
                        value = format!("{}{}", &value, ty);

                        if i < self.len() - 1 {
                            value = value + ", ";
                        }
                    }

                    return crate::TypeId::from_string(value + ")");
                }

                pub fn len(&self) -> usize {
                    return self.0.len();
                }

                pub fn assignable_to(&self, ty: crate::Type) -> bool {
                    return self.id() == ty.id();
                }

                pub fn convertable_to(&self, ty: crate::Type) -> bool {
                    return ty.is_tuple();
                }

                pub fn get(&self) -> [Box<crate::Type>; $len] {
                    return self.0.clone();
                }

                pub fn get_ref(&self) -> &[Box<crate::Type>] {
                    return &self.0;
                }
            }

            impl crate::ToType for $type_name {
                fn to_type(&self) -> crate::Type {
                    return crate::Type::Tuple(crate::TupleType::$name(self.clone()));
                }
            }

            impl std::ops::Index<usize> for $type_name {
                type Output = crate::Type;

                fn index(&self, index: usize) -> &Self::Output {
                    return self.0.index(index);
                }
            }

            impl std::ops::IndexMut<usize> for $type_name {
                fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                    return self.0.index_mut(index);
                }
            }

            impl std::fmt::Display for $type_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    return write!(f, "{}", self.id());
                }
            }
        )*
    };
}

impl<A, B> crate::TypeOf for (A, B)
where
    A: crate::TypeOf,
    B: crate::TypeOf,
{
    fn type_of() -> crate::Type {
        return T2Type::new([Box::new(A::type_of()), Box::new(B::type_of())]).to_type();
    }
}

impl<A, B> crate::ToType for (A, B)
where
    A: crate::TypeOf,
    B: crate::TypeOf,
{
    fn to_type(&self) -> crate::Type {
        return T2Type::new([Box::new(A::type_of()), Box::new(B::type_of())]).to_type();
    }
}

impl<A, B, C> crate::TypeOf for (A, B, C)
where
    A: crate::TypeOf,
    B: crate::TypeOf,
    C: crate::TypeOf,
{
    fn type_of() -> crate::Type {
        return T3Type::new([
            Box::new(A::type_of()),
            Box::new(B::type_of()),
            Box::new(C::type_of()),
        ])
        .to_type();
    }
}

impl<A, B, C> crate::ToType for (A, B, C)
where
    A: crate::TypeOf,
    B: crate::TypeOf,
    C: crate::TypeOf,
{
    fn to_type(&self) -> crate::Type {
        return T3Type::new([
            Box::new(A::type_of()),
            Box::new(B::type_of()),
            Box::new(C::type_of()),
        ])
        .to_type();
    }
}

impl<A, B, C, D> crate::TypeOf for (A, B, C, D)
where
    A: crate::TypeOf,
    B: crate::TypeOf,
    C: crate::TypeOf,
    D: crate::TypeOf,
{
    fn type_of() -> crate::Type {
        return T4Type::new([
            Box::new(A::type_of()),
            Box::new(B::type_of()),
            Box::new(C::type_of()),
            Box::new(D::type_of()),
        ])
        .to_type();
    }
}

impl<A, B, C, D> crate::ToType for (A, B, C, D)
where
    A: crate::TypeOf,
    B: crate::TypeOf,
    C: crate::TypeOf,
    D: crate::TypeOf,
{
    fn to_type(&self) -> crate::Type {
        return T4Type::new([
            Box::new(A::type_of()),
            Box::new(B::type_of()),
            Box::new(C::type_of()),
            Box::new(D::type_of()),
        ])
        .to_type();
    }
}

impl<A, B, C, D, E> crate::TypeOf for (A, B, C, D, E)
where
    A: crate::TypeOf,
    B: crate::TypeOf,
    C: crate::TypeOf,
    D: crate::TypeOf,
    E: crate::TypeOf,
{
    fn type_of() -> crate::Type {
        return T5Type::new([
            Box::new(A::type_of()),
            Box::new(B::type_of()),
            Box::new(C::type_of()),
            Box::new(D::type_of()),
            Box::new(E::type_of()),
        ])
        .to_type();
    }
}

impl<A, B, C, D, E> crate::ToType for (A, B, C, D, E)
where
    A: crate::TypeOf,
    B: crate::TypeOf,
    C: crate::TypeOf,
    D: crate::TypeOf,
    E: crate::TypeOf,
{
    fn to_type(&self) -> crate::Type {
        return T5Type::new([
            Box::new(A::type_of()),
            Box::new(B::type_of()),
            Box::new(C::type_of()),
            Box::new(D::type_of()),
            Box::new(E::type_of()),
        ])
        .to_type();
    }
}

impl<A, B, C, D, E, F> crate::TypeOf for (A, B, C, D, E, F)
where
    A: crate::TypeOf,
    B: crate::TypeOf,
    C: crate::TypeOf,
    D: crate::TypeOf,
    E: crate::TypeOf,
    F: crate::TypeOf,
{
    fn type_of() -> crate::Type {
        return T6Type::new([
            Box::new(A::type_of()),
            Box::new(B::type_of()),
            Box::new(C::type_of()),
            Box::new(D::type_of()),
            Box::new(E::type_of()),
            Box::new(F::type_of()),
        ])
        .to_type();
    }
}

impl<A, B, C, D, E, F> crate::ToType for (A, B, C, D, E, F)
where
    A: crate::TypeOf,
    B: crate::TypeOf,
    C: crate::TypeOf,
    D: crate::TypeOf,
    E: crate::TypeOf,
    F: crate::TypeOf,
{
    fn to_type(&self) -> crate::Type {
        return T6Type::new([
            Box::new(A::type_of()),
            Box::new(B::type_of()),
            Box::new(C::type_of()),
            Box::new(D::type_of()),
            Box::new(E::type_of()),
            Box::new(F::type_of()),
        ])
        .to_type();
    }
}

impl<A, B, C, D, E, F, G> crate::TypeOf for (A, B, C, D, E, F, G)
where
    A: crate::TypeOf,
    B: crate::TypeOf,
    C: crate::TypeOf,
    D: crate::TypeOf,
    E: crate::TypeOf,
    F: crate::TypeOf,
    G: crate::TypeOf,
{
    fn type_of() -> crate::Type {
        return T7Type::new([
            Box::new(A::type_of()),
            Box::new(B::type_of()),
            Box::new(C::type_of()),
            Box::new(D::type_of()),
            Box::new(E::type_of()),
            Box::new(F::type_of()),
            Box::new(G::type_of()),
        ])
        .to_type();
    }
}

impl<A, B, C, D, E, F, G> crate::ToType for (A, B, C, D, E, F, G)
where
    A: crate::TypeOf,
    B: crate::TypeOf,
    C: crate::TypeOf,
    D: crate::TypeOf,
    E: crate::TypeOf,
    F: crate::TypeOf,
    G: crate::TypeOf,
{
    fn to_type(&self) -> crate::Type {
        return T7Type::new([
            Box::new(A::type_of()),
            Box::new(B::type_of()),
            Box::new(C::type_of()),
            Box::new(D::type_of()),
            Box::new(E::type_of()),
            Box::new(F::type_of()),
            Box::new(G::type_of()),
        ])
        .to_type();
    }
}

tuple_type! {
    T1 T1Type to_t1 1
    T2 T2Type to_t2 2
    T3 T3Type to_t3 3
    T4 T4Type to_t4 4
    T5 T5Type to_t5 5
    T6 T6Type to_t6 6
    T7 T7Type to_t7 7
    T8 T8Type to_t8 8
    T9 T9Type to_t9 9
    T10 T10Type to_t10 10
}
