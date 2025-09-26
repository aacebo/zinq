use quote::quote;

use crate::reflect_visibility;

pub fn attr(item: &syn::ItemTrait) -> proc_macro2::TokenStream {
    let name = &item.ident;
    let ty = build(item);

    return quote! {
        #item

        impl ::zinq_reflect::TypeOf for dyn #name {
            fn type_of() -> ::zinq_reflect::Type {
                return #ty;
            }
        }
    };
}

pub fn build(item: &syn::ItemTrait) -> proc_macro2::TokenStream {
    let name = &item.ident;
    let vis = reflect_visibility::build(&item.vis);
    let methods = item.items
        .iter()
        .filter_map(|item| {
            if let syn::TraitItem::Fn(func) = item {
                let fn_name = &func.sig.ident;
                let fn_is_async = match &func.sig.asyncness {
                    None => false,
                    Some(_) => true,
                };

                let fn_return_type = match &func.sig.output {
                    syn::ReturnType::Default => quote!(::zinq_reflect::Type::Void),
                    syn::ReturnType::Type(_, ty) => quote!(::zinq_reflect::type_of!(#ty)),
                };

                let fn_params = func.sig.inputs
                    .iter()
                    .map(|arg| match arg {
                        syn::FnArg::Receiver(recv) => {
                            let mut param_ty = quote! {
                                ::zinq_reflect::SelfType.to_type()
                            };

                            if let Some(_) = &recv.mutability {
                                param_ty = quote!(::zinq_reflect::MutType::new(&#param_ty).to_type());
                            }

                            if let syn::Type::Reference(_) = recv.ty.as_ref() {
                                param_ty = quote!(::zinq_reflect::PtrType::new(&#param_ty).to_type());
                            }

                            quote! {
                                ::zinq_reflect::Param::new(
                                    "self",
                                    &#param_ty,
                                )
                            }
                        },
                        syn::FnArg::Typed(typed) => match typed.pat.as_ref() {
                            syn::Pat::Ident(ident) => {
                                let arg_name = &ident.ident;
                                let arg_ty = &typed.ty;
                                let mut param_ty = quote!(::zinq_reflect::type_of!(#arg_ty));

                                if let Some(_) = &ident.mutability {
                                    param_ty = quote!(::zinq_reflect::MutType::new(&#param_ty).to_type());
                                }

                                if let syn::Type::Ptr(_) = typed.ty.as_ref() {
                                    param_ty = quote!(::zinq_reflect::PtrType::new(&#param_ty).to_type());
                                }

                                quote! {
                                    ::zinq_reflect::Param::new(
                                        stringify!(#arg_name),
                                        &#param_ty,
                                    )
                                }
                            },
                            _ => quote!(),
                        },
                    })
                    .collect::<Vec<_>>();

                return Some(quote! {
                    ::zinq_reflect::Method::new(stringify!(#fn_name))
                        .is_async(#fn_is_async)
                        .visibility(::zinq_reflect::Visibility::Public(::zinq_reflect::Public::Full))
                        .params(&[#(#fn_params,)*])
                        .return_type(&#fn_return_type)
                        .build()
                });
            }

            None
        })
        .collect::<Vec<_>>();

    return quote! {
        ::zinq_reflect::TraitType::new(&(::zinq_reflect::Path::from(module_path!())), stringify!(#name))
            .visibility(#vis)
            .methods(&[#(#methods,)*])
            .build()
            .to_type()
    };
}
