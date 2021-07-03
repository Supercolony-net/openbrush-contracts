extern crate proc_macro;

use quote::{
    quote,
    format_ident,
};
use syn::{TraitItemMethod, ImplItem};
use proc_macro::TokenStream;
use std::collections::HashMap;
use crate::metadata::Metadata;

pub(crate) struct NamedField(syn::Field);

impl syn::parse::Parse for NamedField {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self(syn::Field::parse_named(input)?))
    }
}

impl NamedField {
    pub(crate) fn field(&self) -> &syn::Field {
        &self.0
    }
}

pub(crate) struct Attributes(Vec<syn::Attribute>);

impl syn::parse::Parse for Attributes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self(syn::Attribute::parse_outer(input)?))
    }
}

impl Attributes {
    pub(crate) fn attr(&self) -> &Vec<syn::Attribute> {
        &self.0
    }
}

pub(crate) fn impl_storage_trait(struct_ident: &syn::Ident, trait_ident: &syn::Ident, metadata: &Metadata) -> (Vec<syn::Field>, TokenStream) {
    let trait_methods =
        metadata.storage_traits.get(&trait_ident.to_string()).methods();

    let mut impl_methods = vec![];
    let mut fields = vec![];
    trait_methods.iter().for_each(
        |method| {
            let mut str = method.sig.ident.to_string();
            // skip every not internal method
            if str.chars().next().expect("Method signature is empty") != '_' {
                return
            }
            let mut is_mut = false;
            let mut modifier = quote! { & };
            // remove _ and _mut from name of function to get name of field
            str = str.drain(1..).collect();
            if str.contains("_mut") {
                str = str.drain(..(str.len() - 4 /* sizeof("_mut") */)).collect();
                modifier = quote! { &mut };
                is_mut = true;
            }
            let field_ident = format_ident!("{}", str);
            let method_ty;
            let field_ty;
            if let syn::ReturnType::Type(_, t) = method.sig.output.clone() {
                method_ty = quote! { #t };

                if let syn::Type::Reference(reference) = *t {
                    field_ty = reference.elem;
                } else {
                    panic!("Getter must return references")
                }
            } else {
                panic!("Getter can't return empty object")
            }
            let method_ident = method.sig.ident.clone();

            impl_methods.push(quote! {
                fn #method_ident(#modifier self) -> #method_ty {
                    #modifier self.#field_ident
                }
            });

            // Only add field if we added mut getter
            if is_mut {
                let field_stream = quote! {
                    #[cfg(not(feature = "ink-as-dependency"))]
                    #field_ident: #field_ty
                };

                let field = syn::parse2::<NamedField>(field_stream)
                    .expect("Can't parse custom field");
                fields.push(field.field().clone());
            }
        }
    );

    let code = quote! {
        #[cfg(not(feature = "ink-as-dependency"))]
        impl #trait_ident for #struct_ident {
            #(#impl_methods)*
        }
    };
    (fields, code.into())
}

pub(crate) fn impl_external_trait(impl_item: &mut syn::ItemImpl, trait_ident: &syn::Ident, metadata: &Metadata) -> TokenStream {
    let mut super_methods: HashMap<String, syn::TraitItemMethod> = HashMap::new();
    let mut trait_methods: HashMap<String, syn::TraitItemMethod> = HashMap::new();
    metadata.external_traits.get(&trait_ident.to_string())
        .methods()
        .into_iter()
        .filter_map(|method| {
            trait_methods.insert(method.sig.ident.to_string(), method.clone());
            if method.default.is_some() {
                Some(method)
            } else {
                None
            }
        })
        .for_each(|method: TraitItemMethod| {
            let key = method.sig.ident.to_string();
            super_methods.insert(key, method);
        });

    let mut methods_implemented_by_user: HashMap<String, syn::ImplItemMethod> = HashMap::new();

    // Consume all super calls and add attributes from trait definition
    impl_item.items
        .iter_mut()
        .for_each(|mut item|
            if let syn::ImplItem::Method(method) = &mut item {
                let method_key = method.sig.ident.to_string();
                methods_implemented_by_user.insert(method_key.clone(), method.clone());

                let trait_method = trait_methods.get_mut(&method_key)
                    .expect("Unknown method of trait");
                // Copy attributes from trait definition to user's implementation
                method.attrs.append(&mut trait_method.attrs);

                consume_super_call(method, &super_methods);
            });

    let mut internal_methods: Vec<_> = vec![];
    // Let's create internal `impl section` with default implementation from the trait
    // for method which has been overridden
    let default_impl_of_overridden_methods: Vec<_> = super_methods.clone()
        .into_iter()
        .filter_map(|(k, v)| {
            if methods_implemented_by_user.contains_key(&k) {
                Some(v)
            } else {
                let item = syn::parse2::<ImplItem>(quote! {
                    #v
                }).expect("Can't parse default TraitItemMethod like ImplItem");
                if is_attr(&v.attrs, "ink") {
                    // Let's add default implementation to `impl section`
                    // for methods which are not defined by user
                    impl_item.items.push(item);
                } else {
                    internal_methods.push(item);
                }
                None
            }
        })
        .map(|mut method| {
            // We can remove attributes, because external implementation will use this attributes
            method.attrs.clear();
            // we need to change the naming of internal implementation to "old_name + _name_of_trait"

            method.sig.ident = format_ident!("{}_super", method.sig.ident);
            let item = syn::parse2::<ImplItem>(quote! {
                #method
            }).expect("Can't parse TraitItemMethod like ImplItem");
            item
        }).collect();

    // Now `impl section for trait` can contain not ink functions overridden by user,
    // we need to extract them and put into `impl section`
    let mut overridden_not_ink_methods: Vec<_> = vec![];
    impl_item.items = impl_item.items.clone()
        .into_iter()
        .filter_map(|item|
            if let syn::ImplItem::Method(method) = &item {
                if is_attr(&method.attrs, "ink") {
                    Some(item)
                } else {
                    overridden_not_ink_methods.push(item);
                    None
                }
            } else {
                Some(item)
            })
        .collect();

    // Collect internal method for `impl section`
    let all_internal_methods = vec![
        internal_methods,
        default_impl_of_overridden_methods,
        overridden_not_ink_methods
    ].concat();

    let self_ty = impl_item.self_ty.clone().as_ref().clone();
    let gen = quote! {
        #[cfg(not(feature = "ink-as-dependency"))]
        impl #self_ty {
            #(#all_internal_methods)*
        }
    };
    gen.into()
}

fn consume_super_call(method: &mut syn::ImplItemMethod, super_methods: &HashMap<String, syn::TraitItemMethod>) {
    // Inside of the code of user's implementation, user may want to call base
    // implementation from the trait. It will contains the next syntax "Trait::method()..."
    // We need to change Trait -> Self and method -> method_Trait.
    // method_Trait will be defined in separate `impl section` later
    method.block.stmts
        .iter_mut()
        .filter_map(|stmt|
            if let syn::Stmt::Expr(exp) = stmt {
                Some(exp)
            } else if let syn::Stmt::Semi(exp, _) = stmt {
                Some(exp)
            } else {
                None
            }
        )
        .filter_map(|expr|
            if let syn::Expr::MethodCall(call) = expr {
                Some(call)
            } else {
                None
            }
        ).for_each(|call| {
        // If call contains "super" attribute,
        // when paste the call to trait's implementation
        if is_attr(&call.attrs, "super") {
            assert!(super_methods.contains_key(&call.method.to_string()),
                    "Trait doesn't have default implementation for super call");
            call.method = format_ident!("{}_super", call.method);
            call.attrs = remove_attr(&call.attrs, "super");
        }
    });
}

#[inline]
pub(crate) fn is_attr(attrs: &Vec<syn::Attribute>, ident: &str) -> bool {
    if let None = attrs.iter().find(|attr|
        attr.path.segments.last().expect("No segments in path").ident == ident) {
        false
    } else {
        true
    }
}

#[inline]
pub(crate) fn get_attr(attrs: &Vec<syn::Attribute>, ident: &str) -> Option<syn::Attribute> {
    for attr in attrs.iter() {
        if is_attr(&vec![attr.clone()], ident) {
            return Some(attr.clone())
        }
    }
    None
}

#[inline]
pub(crate) fn remove_attr(attrs: &Vec<syn::Attribute>, ident: &str) -> Vec<syn::Attribute> {
    attrs.clone()
        .into_iter()
        .filter_map(|attr|
            if is_attr(&vec![attr.clone()], ident) {
                None
            } else {
                Some(attr)
            })
        .collect()
}