#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MapType {
    pub(crate) ty: Box<crate::Type>,
    pub(crate) key: Box<crate::Type>,
    pub(crate) value: Box<crate::Type>,
}

impl MapType {
    pub fn new(ty: &crate::Type, key: &crate::Type, value: &crate::Type) -> Self {
        return Self {
            ty: Box::new(ty.clone()),
            key: Box::new(key.clone()),
            value: Box::new(value.clone()),
        };
    }

    pub fn id(&self) -> crate::TypeId {
        return self.ty.id();
    }

    pub fn to_type(&self) -> crate::Type {
        return crate::Type::Map(self.clone());
    }

    pub fn assignable_to(&self, ty: crate::Type) -> bool {
        return self.ty.assignable_to(ty);
    }

    pub fn convertable_to(&self, ty: crate::Type) -> bool {
        return self.ty.convertable_to(ty);
    }
}

impl MapType {
    pub fn meta(&self) -> &crate::MetaData {
        return self.ty.meta();
    }

    pub fn path(&self) -> &crate::Path {
        return self.ty.path();
    }

    pub fn ty(&self) -> &crate::Type {
        return &self.ty;
    }

    pub fn key(&self) -> &crate::Type {
        return &self.key;
    }

    pub fn value(&self) -> &crate::Type {
        return &self.value;
    }
}

impl std::fmt::Display for MapType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", &self.ty);
    }
}

impl<K: crate::TypeOf, V: crate::TypeOf> crate::TypeOf for std::collections::HashMap<K, V> {
    fn type_of() -> crate::Type {
        let path = crate::Path::from("std::collections");
        let key = K::type_of();
        let value = V::type_of();
        let ty = crate::StructType::new(&path, "HashMap")
            .visibility(crate::Visibility::Public(crate::Public::Full))
            .generics(&crate::Generics::from([
                crate::TypeParam::new("K").build().to_generic(),
                crate::TypeParam::new("V").build().to_generic(),
            ]))
            .build()
            .to_type();

        return MapType::new(&ty, &key, &value).to_type();
    }
}

impl<K: crate::TypeOf, V: crate::TypeOf> crate::TypeOf for std::collections::BTreeMap<K, V> {
    fn type_of() -> crate::Type {
        let path = crate::Path::from("std::collections");
        let key = K::type_of();
        let value = V::type_of();
        let ty = crate::StructType::new(&path, "BTreeMap")
            .visibility(crate::Visibility::Public(crate::Public::Full))
            .generics(&crate::Generics::from([
                crate::TypeParam::new("K").build().to_generic(),
                crate::TypeParam::new("V").build().to_generic(),
            ]))
            .build()
            .to_type();

        return MapType::new(&ty, &key, &value).to_type();
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{TypeOf, type_of};

    #[test]
    pub fn type_of() {
        let ty = type_of!(HashMap<String, bool>);

        assert!(ty.is_map());
        assert_eq!(ty.to_map().key(), &type_of!(String));
        assert_eq!(ty.to_map().value(), &type_of!(bool));
    }
}
