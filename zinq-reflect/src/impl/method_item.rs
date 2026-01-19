use crate::{ImplItem, Param, ty::Type};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MethodItem {
    pub name: String,
    pub params: Vec<Param>,
    pub return_ty: Option<Type>,
}

impl From<MethodItem> for ImplItem {
    fn from(value: MethodItem) -> Self {
        Self::Method(value)
    }
}

impl std::fmt::Display for MethodItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fn {}(", &self.name)?;

        for (i, param) in self.params.iter().enumerate() {
            write!(f, "{}", param)?;

            if i < self.params.len() - 1 {
                write!(f, ", ")?;
            }
        }

        write!(f, ")")?;

        if let Some(return_ty) = &self.return_ty {
            write!(f, " -> {}", return_ty)?;
        }

        Ok(())
    }
}
