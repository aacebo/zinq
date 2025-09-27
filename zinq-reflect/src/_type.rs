#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    Bool(crate::BoolType),
    Enum(crate::EnumType),
    Number(crate::NumberType),
    String(crate::StringType),
    Ref(crate::RefType),
    Slice(crate::SliceType),
    Struct(crate::StructType),
    _Self(crate::SelfType),
    Tuple(crate::TupleType),
    Trait(crate::TraitType),
    Mut(crate::MutType),
    Mod(crate::ModType),
    Seq(crate::SeqType),
    Map(crate::MapType),
    Void,
}

impl Type {
    pub fn id(&self) -> crate::TypeId {
        return match self {
            Self::Bool(v) => v.id(),
            Self::Enum(v) => v.id(),
            Self::Number(v) => v.id(),
            Self::String(v) => v.id(),
            Self::Ref(v) => v.id(),
            Self::Slice(v) => v.id(),
            Self::Struct(v) => v.id(),
            Self::_Self(v) => v.id(),
            Self::Tuple(v) => v.id(),
            Self::Trait(v) => v.id(),
            Self::Mut(v) => v.id(),
            Self::Mod(v) => v.id(),
            Self::Seq(v) => v.id(),
            Self::Map(v) => v.id(),
            Self::Void => crate::TypeId::from_str("void"),
        };
    }

    pub fn len(&self) -> usize {
        return match self {
            Self::Enum(v) => v.len(),
            Self::Slice(v) => v.len(),
            Self::Struct(v) => v.len(),
            Self::Tuple(v) => v.len(),
            Self::Trait(v) => v.len(),
            Self::Mod(v) => v.len(),
            Self::Seq(v) => v.len(),
            _ => panic!("called 'len' on '{}'", self.id()),
        };
    }

    pub fn path(&self) -> &crate::Path {
        return match self {
            Self::Struct(v) => v.path(),
            Self::Enum(v) => v.path(),
            Self::Trait(v) => v.path(),
            Self::Mod(v) => v.path(),
            Self::Seq(v) => v.path(),
            Self::Map(v) => v.path(),
            _ => panic!("called 'path' on '{}'", self.id()),
        };
    }

    pub fn meta(&self) -> &crate::MetaData {
        return match self {
            Self::Struct(v) => v.meta(),
            Self::Enum(v) => v.meta(),
            Self::Trait(v) => v.meta(),
            Self::Mod(v) => v.meta(),
            Self::Seq(v) => v.meta(),
            Self::Map(v) => v.meta(),
            _ => panic!("called 'meta' on '{}'", self.id()),
        };
    }

    pub fn to_item(&self) -> crate::Item {
        return crate::Item::Type(self.clone());
    }

    pub fn is_bool(&self) -> bool {
        return match self {
            Self::Bool(_) => true,
            _ => false,
        };
    }

    pub fn is_enum(&self) -> bool {
        return match self {
            Self::Enum(_) => true,
            _ => false,
        };
    }

    pub fn is_ptr(&self) -> bool {
        return match self {
            Self::Ref(_) => true,
            _ => false,
        };
    }

    pub fn is_ptr_of(&self, ty: Self) -> bool {
        return match self {
            Self::Ref(v) => v.is_ptr_of(ty),
            _ => false,
        };
    }

    pub fn is_ptr_self(&self) -> bool {
        return match self {
            Self::Ref(v) => v.ty().is_self(),
            _ => false,
        };
    }

    pub fn is_ptr_mut(&self) -> bool {
        return match self {
            Self::Ref(v) => v.ty().is_mut(),
            _ => false,
        };
    }

    pub fn is_ptr_mut_self(&self) -> bool {
        return match self {
            Self::Ref(v) => v.ty().is_mut_self(),
            _ => false,
        };
    }

    pub fn is_slice(&self) -> bool {
        return match self {
            Self::Slice(_) => true,
            _ => false,
        };
    }

    pub fn is_slice_of(&self, ty: Self) -> bool {
        return match self {
            Self::Slice(v) => v.is_slice_of(ty),
            _ => false,
        };
    }

    pub fn is_struct(&self) -> bool {
        return match self {
            Self::Struct(_) => true,
            _ => false,
        };
    }

    pub fn is_number(&self) -> bool {
        return match self {
            Self::Number(_) => true,
            _ => false,
        };
    }

    pub fn is_int(&self) -> bool {
        return match self {
            Self::Number(v) => v.is_int(),
            _ => false,
        };
    }

    pub fn is_float(&self) -> bool {
        return match self {
            Self::Number(v) => v.is_float(),
            _ => false,
        };
    }

    pub fn is_signed(&self) -> bool {
        return match self {
            Self::Number(v) => v.is_signed(),
            _ => false,
        };
    }

    pub fn is_string(&self) -> bool {
        return match self {
            Self::String(_) => true,
            _ => false,
        };
    }

    pub fn is_self(&self) -> bool {
        return match self {
            Self::_Self(_) => true,
            _ => false,
        };
    }

    pub fn is_tuple(&self) -> bool {
        return match self {
            Self::Tuple(_) => true,
            _ => false,
        };
    }

    pub fn is_trait(&self) -> bool {
        return match self {
            Self::Trait(_) => true,
            _ => false,
        };
    }

    pub fn is_mut(&self) -> bool {
        return match self {
            Self::Mut(_) => true,
            _ => false,
        };
    }

    pub fn is_mut_of(&self, ty: crate::Type) -> bool {
        return match self {
            Self::Mut(v) => v.is_mut_of(ty),
            _ => false,
        };
    }

    pub fn is_mut_self(&self) -> bool {
        return match self {
            Self::Mut(v) => v.ty().is_self(),
            _ => false,
        };
    }

    pub fn is_mod(&self) -> bool {
        return match self {
            Self::Mod(_) => true,
            _ => false,
        };
    }

    pub fn is_seq(&self) -> bool {
        return match self {
            Self::Seq(_) => true,
            _ => false,
        };
    }

    pub fn is_map(&self) -> bool {
        return match self {
            Self::Map(_) => true,
            _ => false,
        };
    }

    pub fn is_void(&self) -> bool {
        return match self {
            Self::Void => true,
            _ => false,
        };
    }

    pub fn to_bool(&self) -> crate::BoolType {
        return match self {
            Self::Bool(v) => v.clone(),
            _ => panic!("called 'to_bool' on '{}'", self.id()),
        };
    }

    pub fn to_enum(&self) -> crate::EnumType {
        return match self {
            Self::Enum(v) => v.clone(),
            _ => panic!("called 'to_enum' on '{}'", self.id()),
        };
    }

    pub fn to_ptr(&self) -> crate::RefType {
        return match self {
            Self::Ref(v) => v.clone(),
            _ => panic!("called 'to_ptr' on '{}'", self.id()),
        };
    }

    pub fn to_slice(&self) -> crate::SliceType {
        return match self {
            Self::Slice(v) => v.clone(),
            _ => panic!("called 'to_slice' on '{}'", self.id()),
        };
    }

    pub fn to_struct(&self) -> crate::StructType {
        return match self {
            Self::Struct(v) => v.clone(),
            _ => panic!("called 'to_struct' on '{}'", self.id()),
        };
    }

    pub fn to_number(&self) -> crate::NumberType {
        return match self {
            Self::Number(v) => v.clone(),
            _ => panic!("called 'to_number' on '{}'", self.id()),
        };
    }

    pub fn to_int(&self) -> crate::IntType {
        return match self {
            Self::Number(v) => v.to_int(),
            _ => panic!("called 'to_int' on '{}'", self.id()),
        };
    }

    pub fn to_float(&self) -> crate::FloatType {
        return match self {
            Self::Number(v) => v.to_float(),
            _ => panic!("called 'to_float' on '{}'", self.id()),
        };
    }

    pub fn to_string(&self) -> crate::StringType {
        return match self {
            Self::String(v) => v.clone(),
            _ => panic!("called 'to_string' on '{}'", self.id()),
        };
    }

    pub fn to_self(&self) -> crate::SelfType {
        return match self {
            Self::_Self(v) => v.clone(),
            _ => panic!("called 'to_self' on '{}'", self.id()),
        };
    }

    pub fn to_tuple(&self) -> crate::TupleType {
        return match self {
            Self::Tuple(v) => v.clone(),
            _ => panic!("called 'to_tuple' on '{}'", self.id()),
        };
    }

    pub fn to_trait(&self) -> crate::TraitType {
        return match self {
            Self::Trait(v) => v.clone(),
            _ => panic!("called 'to_trait' on '{}'", self.id()),
        };
    }

    pub fn to_mut(&self) -> crate::MutType {
        return match self {
            Self::Mut(v) => v.clone(),
            _ => panic!("called 'to_mut' on '{}'", self.id()),
        };
    }

    pub fn to_mod(&self) -> crate::ModType {
        return match self {
            Self::Mod(v) => v.clone(),
            _ => panic!("called 'to_mod' on '{}'", self.id()),
        };
    }

    pub fn to_seq(&self) -> crate::SeqType {
        return match self {
            Self::Seq(v) => v.clone(),
            _ => panic!("called 'to_seq' on '{}'", self.id()),
        };
    }

    pub fn to_map(&self) -> crate::MapType {
        return match self {
            Self::Map(v) => v.clone(),
            _ => panic!("called 'to_map' on '{}'", self.id()),
        };
    }

    pub fn assignable_to(&self, ty: Self) -> bool {
        return match self {
            Self::Bool(v) => v.assignable_to(ty),
            Self::Enum(v) => v.assignable_to(ty),
            Self::Number(v) => v.assignable_to(ty),
            Self::String(v) => v.assignable_to(ty),
            Self::Ref(v) => v.assignable_to(ty),
            Self::Slice(v) => v.assignable_to(ty),
            Self::Struct(v) => v.assignable_to(ty),
            Self::_Self(v) => v.assignable_to(ty),
            Self::Tuple(v) => v.assignable_to(ty),
            Self::Trait(v) => v.assignable_to(ty),
            Self::Mut(v) => v.assignable_to(ty),
            Self::Mod(v) => v.assignable_to(ty),
            Self::Seq(v) => v.assignable_to(ty),
            Self::Map(v) => v.assignable_to(ty),
            Self::Void => false,
        };
    }

    pub fn convertable_to(&self, ty: Self) -> bool {
        return match self {
            Self::Bool(v) => v.convertable_to(ty),
            Self::Enum(v) => v.convertable_to(ty),
            Self::Number(v) => v.convertable_to(ty),
            Self::String(v) => v.convertable_to(ty),
            Self::Ref(v) => v.convertable_to(ty),
            Self::Slice(v) => v.convertable_to(ty),
            Self::Struct(v) => v.convertable_to(ty),
            Self::_Self(v) => v.convertable_to(ty),
            Self::Tuple(v) => v.convertable_to(ty),
            Self::Trait(v) => v.convertable_to(ty),
            Self::Mut(v) => v.convertable_to(ty),
            Self::Mod(v) => v.convertable_to(ty),
            Self::Seq(v) => v.convertable_to(ty),
            Self::Map(v) => v.convertable_to(ty),
            Self::Void => false,
        };
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Self::Bool(v) => write!(f, "{}", v),
            Self::Enum(v) => write!(f, "{}", v),
            Self::Number(v) => write!(f, "{}", v),
            Self::String(v) => write!(f, "{}", v),
            Self::Ref(v) => write!(f, "{}", v),
            Self::Slice(v) => write!(f, "{}", v),
            Self::Struct(v) => write!(f, "{}", v),
            Self::_Self(v) => write!(f, "{}", v),
            Self::Tuple(v) => write!(f, "{}", v),
            Self::Trait(v) => write!(f, "{}", v),
            Self::Mut(v) => write!(f, "{}", v),
            Self::Mod(v) => write!(f, "{}", v),
            Self::Seq(v) => write!(f, "{}", v),
            Self::Map(v) => write!(f, "{}", v),
            Self::Void => write!(f, "void"),
        };
    }
}
