use quote::quote;

use crate::{reflect_field, reflect_generics, reflect_meta, reflect_visibility};

pub fn derive(input: &syn::DeriveInput, data: &syn::DataStruct) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let ty = build(&syn::ItemStruct {
        attrs: input.attrs.clone(),
        fields: data.fields.clone(),
        generics: input.generics.clone(),
        ident: input.ident.clone(),
        semi_token: data.semi_token.clone(),
        struct_token: data.struct_token.clone(),
        vis: input.vis.clone(),
    });

    let fields = match &data.fields {
        syn::Fields::Named(named_fields) => named_fields
            .named
            .iter()
            .map(|field| {
                let field_ident = &field.ident;
                quote!(#field_ident)
            })
            .collect::<Vec<_>>(),
        syn::Fields::Unnamed(unnamed_fields) => unnamed_fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let field_ident = syn::Member::Unnamed(syn::Index {
                    index: i as u32,
                    span: proc_macro2::Span::call_site(),
                });

                quote!(#field_ident)
            })
            .collect::<Vec<_>>(),
        syn::Fields::Unit => vec![],
    };

    return quote! {
        impl ::reflectx::TypeOf for #name {
            fn type_of() -> ::reflectx::Type {
                return #ty;
            }
        }

        impl ::reflectx::ToType for #name {
            fn to_type(&self) -> ::reflectx::Type {
                return #ty;
            }
        }

        impl ::reflectx::ToValue for #name {
            fn to_value(self) -> ::reflectx::Value {
                return ::reflectx::Dynamic::from_object(self).to_value();
            }
        }

        impl ::reflectx::AsValue for #name {
            fn as_value(&self) -> ::reflectx::Value {
                return ::reflectx::Dynamic::from_object(self.clone()).as_value();
            }
        }

        impl ::reflectx::Dyn for #name { }

        impl ::reflectx::Object for #name {
            fn field(&self, name: &::reflectx::FieldName) -> ::reflectx::Value {
                #(
                    if name == stringify!(#fields) {
                        return ::reflectx::value_of!(self.#fields.clone());
                    }
                )*

                return ::reflectx::Value::Null;
            }
        }
    };
}

pub fn attr(item: &syn::ItemStruct) -> proc_macro2::TokenStream {
    let name = &item.ident;
    let ty = build(item);
    let fields = match &item.fields {
        syn::Fields::Named(named_fields) => named_fields
            .named
            .iter()
            .map(|field| {
                let field_ident = &field.ident;
                quote!(#field_ident)
            })
            .collect::<Vec<_>>(),
        syn::Fields::Unnamed(unnamed_fields) => unnamed_fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let field_ident = syn::Member::Unnamed(syn::Index {
                    index: i as u32,
                    span: proc_macro2::Span::call_site(),
                });

                quote!(#field_ident)
            })
            .collect::<Vec<_>>(),
        syn::Fields::Unit => vec![],
    };

    return quote! {
        impl ::reflectx::TypeOf for #name {
            fn type_of() -> ::reflectx::Type {
                return #ty;
            }
        }

        impl ::reflectx::ToType for #name {
            fn to_type(&self) -> ::reflectx::Type {
                return #ty;
            }
        }

        impl ::reflectx::ToValue for #name {
            fn to_value(self) -> ::reflectx::Value {
                return ::reflectx::Dynamic::from_object(self).to_value();
            }
        }

        impl ::reflectx::AsValue for #name {
            fn as_value(&self) -> ::reflectx::Value {
                return ::reflectx::Dynamic::from_object(self.clone()).as_value();
            }
        }

        impl ::reflectx::Dyn for #name { }

        impl ::reflectx::Object for #name {
            fn field(&self, name: &::reflectx::FieldName) -> ::reflectx::Value {
                #(
                    if name == stringify!(#fields) {
                        return ::reflectx::value_of!(self.#fields.clone());
                    }
                )*

                return ::reflectx::Value::Null;
            }
        }
    };
}

pub fn build(item: &syn::ItemStruct) -> proc_macro2::TokenStream {
    let name = &item.ident;
    let vis = reflect_visibility::build(&item.vis);
    let meta = reflect_meta::build(&item.attrs);
    let generics = reflect_generics::build(&item.generics);
    let layout = match &item.fields {
        syn::Fields::Named(_) => quote!(::reflectx::Layout::Key),
        syn::Fields::Unnamed(_) => quote!(::reflectx::Layout::Index),
        syn::Fields::Unit => quote!(::reflectx::Layout::Unit),
    };

    let fields = match &item.fields {
        syn::Fields::Named(named_fields) => named_fields
            .named
            .iter()
            .enumerate()
            .map(|(i, field)| reflect_field::build(field, i, true))
            .collect::<Vec<_>>(),
        syn::Fields::Unnamed(unnamed_fields) => unnamed_fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(i, field)| reflect_field::build(field, i, false))
            .collect::<Vec<_>>(),
        syn::Fields::Unit => vec![],
    };

    return quote! {
        ::reflectx::StructType::new()
            .with_path(&(::reflectx::Path::from(module_path!())))
            .with_name(stringify!(#name))
            .with_visibility(#vis)
            .with_meta(&#meta)
            .with_generics(&#generics)
            .with_fields(
                ::reflectx::Fields::new()
                    .with_layout(#layout)
                    .with_fields(&[#(#fields,)*])
                    .build()
                    .as_ref()
            )
            .build()
            .to_type()
    };
}
