use std::sync::Arc;

/// ## Sequence
///
/// implemented by types that
/// can reflect their value/type and that
/// of their individual index's in a sequence
pub trait Sequence: std::any::Any + std::fmt::Debug + crate::ToType + crate::ToValue {
    fn len(&self) -> usize;
    fn index(&self, i: usize) -> &crate::Value;
}

impl dyn Sequence {
    pub fn downcast_ref<T: std::any::Any>(&self) -> Option<&T> {
        let value: &dyn std::any::Any = self;
        return value.downcast_ref::<T>();
    }

    pub fn downcast_mut<T: std::any::Any>(&mut self) -> Option<&mut T> {
        let value: &mut dyn std::any::Any = self;
        return value.downcast_mut::<T>();
    }

    pub fn is<T: std::any::Any>(&self) -> bool {
        let value: &dyn std::any::Any = self;
        return value.is::<T>();
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for dyn Sequence {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;

        let ty = self.to_type().to_slice();
        let mut ser = serializer.serialize_seq(ty.capacity())?;

        for i in 0..self.len() {
            ser.serialize_element(self.index(i))?;
        }

        return ser.end();
    }
}

impl std::fmt::Display for dyn Sequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        for i in 0..self.len() {
            write!(f, "\n\t{}", self.index(i))?;
        }

        return write!(f, "\n]");
    }
}

impl<T: Clone + Sequence> Sequence for Arc<T> {
    fn len(&self) -> usize {
        return self.as_ref().len();
    }

    fn index(&self, i: usize) -> &crate::Value {
        return self.as_ref().index(i);
    }
}
