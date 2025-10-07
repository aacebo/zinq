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
        impl ::zinq_reflect::TypeOf for #name {
            fn type_of() -> ::zinq_reflect::Type {
                return #ty;
            }
        }

        impl ::zinq_reflect::ToType for #name {
            fn to_type(&self) -> ::zinq_reflect::Type {
                return #ty;
            }
        }

        impl ::zinq_reflect::ToValue for #name {
            fn to_value(self) -> ::zinq_reflect::Value {
                return ::zinq_reflect::Dynamic::from_object(self).to_value();
            }
        }

        impl ::zinq_reflect::AsValue for #name {
            fn as_value(&self) -> ::zinq_reflect::Value {
                return ::zinq_reflect::Dynamic::from_object(self.clone()).as_value();
            }
        }

        impl ::zinq_reflect::Dyn for #name { }

        impl ::zinq_reflect::Object for #name {
            fn field(&self, name: &::zinq_reflect::FieldName) -> ::zinq_reflect::Value {
                #(
                    if name == stringify!(#fields) {
                        return ::zinq_reflect::value_of!(self.#fields.clone());
                    }
                )*

                return ::zinq_reflect::Value::Null;
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
        impl ::zinq_reflect::TypeOf for #name {
            fn type_of() -> ::zinq_reflect::Type {
                return #ty;
            }
        }

        impl ::zinq_reflect::ToType for #name {
            fn to_type(&self) -> ::zinq_reflect::Type {
                return #ty;
            }
        }

        impl ::zinq_reflect::ToValue for #name {
            fn to_value(self) -> ::zinq_reflect::Value {
                return ::zinq_reflect::Dynamic::from_object(self).to_value();
            }
        }

        impl ::zinq_reflect::AsValue for #name {
            fn as_value(&self) -> ::zinq_reflect::Value {
                return ::zinq_reflect::Dynamic::from_object(self.clone()).as_value();
            }
        }

        impl ::zinq_reflect::Dyn for #name { }

        impl ::zinq_reflect::Object for #name {
            fn field(&self, name: &::zinq_reflect::FieldName) -> ::zinq_reflect::Value {
                #(
                    if name == stringify!(#fields) {
                        return ::zinq_reflect::value_of!(self.#fields.clone());
                    }
                )*

                return ::zinq_reflect::Value::Null;
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
        syn::Fields::Named(_) => quote!(::zinq_reflect::Layout::Key),
        syn::Fields::Unnamed(_) => quote!(::zinq_reflect::Layout::Index),
        syn::Fields::Unit => quote!(::zinq_reflect::Layout::Unit),
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
        ::zinq_reflect::StructType::new(&(::zinq_reflect::Path::from(module_path!())), stringify!(#name))
            .visibility(#vis)
            .meta(&#meta)
            .generics(&#generics)
            .fields(
                ::zinq_reflect::Fields::new()
                    .layout(#layout)
                    .fields(&[#(#fields,)*])
                    .build()
                    .as_ref()
            )
            .build()
            .to_type()
    };
}
