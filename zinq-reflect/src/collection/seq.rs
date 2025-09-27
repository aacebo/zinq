#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SeqType {
    pub(crate) ty: Box<crate::Type>,
    pub(crate) elem: Box<crate::Type>,
    pub(crate) capacity: Option<usize>,
    pub(crate) contiguous: bool,
    pub(crate) growable: bool,
}

impl SeqType {
    pub fn new(ty: &crate::Type, elem: &crate::Type) -> crate::build::SeqTypeBuilder {
        return crate::build::SeqTypeBuilder::new(ty, elem);
    }

    pub fn id(&self) -> crate::TypeId {
        return self.ty.id();
    }

    pub fn len(&self) -> usize {
        return match self.capacity {
            None => panic!("called 'len' on type '{}'", self.id()),
            Some(v) => v,
        };
    }

    pub fn to_type(&self) -> crate::Type {
        return crate::Type::Seq(self.clone());
    }

    pub fn assignable_to(&self, ty: crate::Type) -> bool {
        return self.ty.assignable_to(ty);
    }

    pub fn convertable_to(&self, ty: crate::Type) -> bool {
        return self.ty.convertable_to(ty);
    }
}

impl SeqType {
    pub fn meta(&self) -> &crate::MetaData {
        return self.ty.meta();
    }

    pub fn path(&self) -> &crate::Path {
        return self.ty.path();
    }

    pub fn ty(&self) -> &crate::Type {
        return &self.ty;
    }

    pub fn elem(&self) -> &crate::Type {
        return &self.elem;
    }

    pub fn capacity(&self) -> Option<usize> {
        return self.capacity;
    }

    pub fn contiguous(&self) -> bool {
        return self.contiguous;
    }

    pub fn growable(&self) -> bool {
        return self.growable;
    }
}

impl std::fmt::Display for SeqType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", &self.ty);
    }
}

impl<T: crate::TypeOf> crate::TypeOf for Vec<T> {
    fn type_of() -> crate::Type {
        let path = crate::Path::from("std::vec");
        let elem = T::type_of();
        let ty = crate::StructType::new(&path, "Vec")
            .visibility(crate::Visibility::Public(crate::Public::Full))
            .generics(&crate::Generics::from([crate::TypeParam::new("T")
                .build()
                .to_generic()]))
            .build()
            .to_type();

        return SeqType::new(&ty, &elem)
            .contiguous(true)
            .growable(true)
            .build()
            .to_type();
    }
}

#[cfg(test)]
mod test {
    use crate::{TypeOf, type_of};

    #[test]
    pub fn type_of() {
        let ty = type_of!(Vec<bool>);

        assert!(ty.is_seq());
        assert_eq!(ty.to_seq().elem(), &type_of!(bool));
    }
}
