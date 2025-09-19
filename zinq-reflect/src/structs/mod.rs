pub mod members;

pub use members::*;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StructType {
    name: String,
    members: Vec<Member>,
}

impl StructType {
    pub fn new(name: &str, members: &[Member]) -> Self {
        return Self {
            name: name.to_string(),
            members: members.to_vec(),
        };
    }

    pub fn id(&self) -> crate::TypeId {
        return crate::TypeId::from_str(&self.name);
    }

    pub fn len(&self) -> usize {
        return self.members.len();
    }

    pub fn to_type(&self) -> crate::Type {
        return crate::Type::Struct(self.clone());
    }

    pub fn assignable_to(&self, ty: crate::Type) -> bool {
        return self.id() == ty.id();
    }

    pub fn convertable_to(&self, ty: crate::Type) -> bool {
        return ty.is_bool();
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Member> {
        return self.members.iter();
    }

    pub fn has_member(&self, name: &str) -> bool {
        return self.members.iter().any(|v| v.name() == name);
    }

    pub fn member(&self, name: &str) -> &Member {
        return self.members.iter().find(|v| v.name() == name).unwrap();
    }

    pub fn member_mut(&mut self, name: &str) -> &mut Member {
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
