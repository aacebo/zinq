use std::any::type_name_of_val;

pub trait Ident {
    fn ident(&self) -> &syn::Ident;
}

impl Ident for syn::Item {
    fn ident(&self) -> &syn::Ident {
        return match self {
            syn::Item::Struct(v) => &v.ident,
            syn::Item::Enum(v) => &v.ident,
            syn::Item::Fn(v) => &v.sig.ident,
            syn::Item::Trait(v) => &v.ident,
            syn::Item::Mod(v) => &v.ident,
            syn::Item::Union(v) => &v.ident,
            syn::Item::ExternCrate(v) => &v.ident,
            syn::Item::Const(v) => &v.ident,
            syn::Item::Static(v) => &v.ident,
            syn::Item::TraitAlias(v) => &v.ident,
            syn::Item::Type(v) => &v.ident,
            syn::Item::Macro(v) => match &v.ident {
                None => panic!("macro ident not found"),
                Some(v) => v,
            },
            other => panic!("type {} is not supported", type_name_of_val(other)),
        };
    }
}

impl Ident for syn::Type {
    fn ident(&self) -> &syn::Ident {
        return match self {
            syn::Type::Path(v) => v.path.require_ident().unwrap(),
            syn::Type::Reference(v) => v.elem.ident(),
            other => panic!("type {} is not supported", type_name_of_val(other)),
        };
    }
}
