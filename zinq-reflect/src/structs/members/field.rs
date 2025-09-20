use crate::Visibility;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Field {
    vis: Visibility,
    name: String,
    ty: Box<crate::Type>,
}

impl Field {
    pub fn new(vis: Visibility, name: &str, ty: &crate::Type) -> Self {
        return Self {
            vis,
            name: name.to_string(),
            ty: Box::from(ty.clone()),
        };
    }

    pub fn vis(&self) -> Visibility {
        return self.vis.clone();
    }

    pub fn name(&self) -> &str {
        return &self.name;
    }

    pub fn ty(&self) -> &crate::Type {
        return &self.ty;
    }
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.vis != Visibility::Private {
            write!(f, "{} ", &self.vis)?;
        }

        return write!(f, "{}: {},", &self.name, &self.ty);
    }
}
