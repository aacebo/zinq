#[derive(Debug, Clone)]
pub struct SeqTypeBuilder(crate::SeqType);

impl SeqTypeBuilder {
    pub fn new(ty: &crate::Type, elem: &crate::Type) -> Self {
        return Self(crate::SeqType {
            ty: Box::new(ty.clone()),
            elem: Box::new(elem.clone()),
            capacity: None,
            contiguous: false,
            growable: false,
        });
    }

    pub fn capacity(&self, capacity: usize) -> Self {
        let mut next = self.clone();
        next.0.capacity = Some(capacity);
        return next;
    }

    pub fn contiguous(&self, contiguous: bool) -> Self {
        let mut next = self.clone();
        next.0.contiguous = contiguous;
        return next;
    }

    pub fn growable(&self, growable: bool) -> Self {
        let mut next = self.clone();
        next.0.growable = growable;
        return next;
    }

    pub fn build(&self) -> crate::SeqType {
        return self.0.clone();
    }
}
