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

    pub fn to_type(&self) -> crate::Type {
        return crate::Type::Map(self.clone());
    }

    pub fn id(&self) -> crate::TypeId {
        return self.ty.id();
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

impl crate::ToType for MapType {
    fn to_type(&self) -> crate::Type {
        return crate::Type::Map(self.clone());
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

impl<K: crate::TypeOf, V: crate::TypeOf> crate::ToType for std::collections::HashMap<K, V> {
    fn to_type(&self) -> crate::Type {
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

impl<K: crate::TypeOf, V: crate::TypeOf> crate::ToType for std::collections::BTreeMap<K, V> {
    fn to_type(&self) -> crate::Type {
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

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Map {
    ty: MapType,
    data: std::collections::BTreeMap<crate::Value, crate::Value>,
}

impl Map {
    pub fn new(ty: &MapType) -> Self {
        return Self {
            ty: ty.clone(),
            data: std::collections::BTreeMap::new(),
        };
    }

    pub fn to_type(&self) -> crate::Type {
        return crate::Type::Map(self.ty.clone());
    }

    pub fn iter(&self) -> std::collections::btree_map::Iter<'_, crate::Value, crate::Value> {
        return self.data.iter();
    }

    pub fn keys(&self) -> Vec<crate::Value> {
        return self.data.clone().into_keys().collect::<Vec<_>>();
    }

    pub fn values(&self) -> Vec<crate::Value> {
        return self.data.clone().into_values().collect::<Vec<_>>();
    }

    pub fn len(&self) -> usize {
        return self.data.len();
    }

    pub fn has(&self, key: &crate::Value) -> bool {
        return self.data.contains_key(key);
    }

    pub fn get(&self, key: &crate::Value) -> Option<&crate::Value> {
        return self.data.get(key);
    }

    pub fn get_mut(&mut self, key: &crate::Value) -> Option<&mut crate::Value> {
        return self.data.get_mut(key);
    }

    pub fn set(&mut self, key: &crate::Value, value: &crate::Value) {
        self.data.insert(key.clone(), value.clone());
    }
}

impl crate::ToType for Map {
    fn to_type(&self) -> crate::Type {
        return crate::Type::Map(self.ty.clone());
    }
}

impl crate::ToValue for Map {
    fn to_value(self) -> crate::Value {
        return crate::Value::Map(self.clone());
    }
}

impl std::ops::Index<&crate::Value> for Map {
    type Output = crate::Value;

    fn index(&self, index: &crate::Value) -> &Self::Output {
        return self.data.index(index);
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;

        for (key, value) in &self.data {
            write!(f, "\n\t{}: {}", key, value)?;
        }

        if self.data.len() > 0 {
            write!(f, "\n")?;
        }

        return write!(f, "}}");
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
