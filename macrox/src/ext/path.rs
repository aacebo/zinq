use std::any::type_name_of_val;

pub trait Path {
    fn path(&self) -> &syn::Path;
}

impl Path for syn::Type {
    fn path(&self) -> &syn::Path {
        return match self {
            syn::Type::Path(v) => &v.path,
            syn::Type::Reference(v) => v.elem.path(),
            other => panic!("type {} is not supported", type_name_of_val(other)),
        };
    }
}
