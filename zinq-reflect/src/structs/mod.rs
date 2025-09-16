pub mod members;

pub use members::*;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StructType {
    name: String,
    members: Vec<MemberType>,
}

impl StructType {
    pub fn id(&self) -> std::any::TypeId {
        return std::any::TypeId::of::<bool>();
    }

    pub fn name(&self) -> &str {
        return &self.name;
    }

    pub fn len(&self) -> usize {
        return self.members.len();
    }

    pub fn to_type(&self) -> crate::Type {
        return crate::Type::Struct(self.clone());
    }

    pub fn assignable_to(&self, _type: crate::Type) -> bool {
        return self.id() == _type.id();
    }

    pub fn convertable_to(&self, _type: crate::Type) -> bool {
        return _type.is_bool();
    }

    pub fn iter(&self) -> std::slice::Iter<'_, MemberType> {
        return self.members.iter();
    }

    pub fn has_member(&self, name: &str) -> bool {
        return self.members.iter().any(|v| v.name() == name);
    }

    pub fn member(&self, name: &str) -> &MemberType {
        return self.members.iter().find(|v| v.name() == name).unwrap();
    }

    pub fn member_mut(&mut self, name: &str) -> &mut MemberType {
        return self.members.iter_mut().find(|v| v.name() == name).unwrap();
    }
}

impl std::fmt::Display for StructType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "struct {} {{", &self.name)?;

        for member in &self.members {
            write!(f, "\n\t{}", member)?;
        }

        if self.members.len() > 0 {
            write!(f, "\n")?;
        }

        return write!(f, "}}");
    }
}
