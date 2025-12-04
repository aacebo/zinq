use std::any::type_name_of_val;

pub trait Attrs {
    fn attrs(&self) -> &[syn::Attribute];
}

impl Attrs for syn::Item {
    fn attrs(&self) -> &[syn::Attribute] {
        return match self {
            syn::Item::Struct(item) => &item.attrs,
            syn::Item::Trait(item) => &item.attrs,
            syn::Item::Enum(item) => &item.attrs,
            syn::Item::Fn(item) => &item.attrs,
            syn::Item::Const(item) => &item.attrs,
            syn::Item::Static(item) => &item.attrs,
            syn::Item::ExternCrate(item) => &item.attrs,
            syn::Item::ForeignMod(item) => &item.attrs,
            syn::Item::Impl(item) => &item.attrs,
            syn::Item::Macro(item) => &item.attrs,
            syn::Item::TraitAlias(item) => &item.attrs,
            syn::Item::Type(item) => &item.attrs,
            syn::Item::Union(item) => &item.attrs,
            syn::Item::Use(item) => &item.attrs,
            syn::Item::Mod(item) => &item.attrs,
            other => panic!("type {} is not supported", type_name_of_val(other)),
        };
    }
}
