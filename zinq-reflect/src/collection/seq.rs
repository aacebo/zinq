use crate::ToType;

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

    pub fn to_type(&self) -> crate::Type {
        return crate::Type::Seq(self.clone());
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

impl crate::ToType for SeqType {
    fn to_type(&self) -> crate::Type {
        return crate::Type::Seq(self.clone());
    }
}

impl<T: crate::TypeOf> crate::ToType for Vec<T> {
    fn to_type(&self) -> crate::Type {
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

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Seq {
    ty: SeqType,
    data: Vec<crate::Value>,
}

impl Seq {
    pub fn new(ty: &SeqType) -> Self {
        return Self {
            ty: ty.clone(),
            data: vec![],
        };
    }

    pub fn to_type(&self) -> crate::Type {
        return crate::Type::Seq(self.ty.clone());
    }

    pub fn iter(&self) -> std::slice::Iter<'_, crate::Value> {
        return self.data.iter();
    }

    pub fn len(&self) -> usize {
        return self.data.len();
    }

    pub fn data(&self) -> &[crate::Value] {
        return &self.data;
    }

    pub fn get(&self, index: usize) -> Option<&crate::Value> {
        return self.data.get(index);
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut crate::Value> {
        return self.data.get_mut(index);
    }

    pub fn set(&mut self, value: crate::Value) {
        self.data = value.to_seq().data.clone();
    }

    pub fn set_index(&mut self, index: usize, value: crate::Value) {
        self.data[index] = value;
    }

    pub fn push(&mut self, value: crate::Value) {
        self.data.push(value);
    }
}

impl crate::ToType for Seq {
    fn to_type(&self) -> crate::Type {
        return crate::Type::Seq(self.ty.clone());
    }
}

impl crate::ToValue for Seq {
    fn to_value(self) -> crate::Value {
        return crate::Value::Seq(self.clone());
    }
}

impl std::ops::Index<usize> for Seq {
    type Output = crate::Value;

    fn index(&self, index: usize) -> &Self::Output {
        return self.data.index(index);
    }
}

impl std::ops::Index<&crate::Value> for Seq {
    type Output = crate::Value;

    fn index(&self, index: &crate::Value) -> &Self::Output {
        return self.data.index(index.to_u32().get() as usize);
    }
}

impl std::ops::IndexMut<usize> for Seq {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return self.data.index_mut(index);
    }
}

impl std::ops::IndexMut<&crate::Value> for Seq {
    fn index_mut(&mut self, index: &crate::Value) -> &mut Self::Output {
        return self.data.index_mut(index.to_u32().get() as usize);
    }
}

impl std::fmt::Display for Seq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        for (i, value) in self.data.iter().enumerate() {
            write!(f, "{}", value)?;

            if i < self.data.len() - 1 {
                write!(f, ", ")?;
            }
        }

        return write!(f, "]");
    }
}

impl<T: crate::TypeOf + crate::ToValue> crate::ToValue for Vec<T> {
    fn to_value(self) -> crate::Value {
        let ty = self.to_type();
        let mut value = Seq::new(ty.as_seq());

        for v in self {
            value.push(v.to_value());
        }

        return value.to_value();
    }
}

#[cfg(test)]
mod test {
    use crate::{TypeOf, type_of, value_of};

    #[test]
    pub fn type_of() {
        let ty = type_of!(Vec<bool>);

        assert!(ty.is_seq());
        assert_eq!(ty.to_seq().elem(), &type_of!(bool));
    }

    #[test]
    pub fn to_value() {
        let value = value_of!(vec![30, 40, 50]);

        assert!(value.is_seq());
        assert_eq!(value.len(), 3);
        assert_eq!(value[1], value_of!(40));
    }
}
