extern crate proc_macro;

use quote::{
    quote,
    format_ident,
};
use syn::{TraitItem, ItemTrait, TraitItemMethod, ImplItem};
use proc_macro::TokenStream;
use proc_macro2::{
    TokenStream as TokenStream2,
};
use std::collections::HashMap;
use std::env;
use std::fs::{OpenOptions, File};
use std::io::{BufReader, Seek, SeekFrom};
use std::str::FromStr;
use serde::{Serialize, Deserialize};
use serde_json;
use fs2::FileExt;
use cargo_metadata::{MetadataCommand};
use std::path::PathBuf;

const TEMP_FILE: &str = "brush_metadata";
#[derive(Default, Debug, Serialize, Deserialize)]
pub(crate) struct TraitDefinitions(HashMap<String, String>);

impl std::ops::Deref for TraitDefinitions {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for TraitDefinitions {
    fn deref_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.0
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub(crate) struct Metadata {
    pub storage_traits: TraitDefinitions,
    pub external_traits: TraitDefinitions,
}

impl Metadata {
    pub(crate) fn load(file: &File) -> Metadata {
        let reader = BufReader::new(file);

        let map = serde_json::from_reader(reader).unwrap_or_default();
        map
    }

    pub(crate) fn save_and_unlock(&self, mut locked_file: File) {
        locked_file.set_len(0).expect("Can't truncate the file");
        locked_file.seek(SeekFrom::Start(0)).expect("Can't set cursor position");
        serde_json::to_writer(&locked_file, self).expect("Can't dump definition metadata to file");
        locked_file.unlock().expect("Can't remove exclusive lock");
    }
}

pub(crate) struct TraitDefinition(ItemTrait);

impl TraitDefinition {
    pub(crate) fn methods(&self) -> Vec<syn::TraitItemMethod> {
        self.0.items
            .clone()
            .into_iter()
            .filter_map(|item| {
                if let TraitItem::Method(method) = item {
                    Some(method)
                } else {
                    None
                }
            }).collect()
    }
}

impl TraitDefinitions {
    pub(crate) fn get(&self, ident: &String) -> TraitDefinition {
        let stream = TokenStream2::from_str(
            self.0.get(ident).expect("Can't find definition of trait")
        ).expect("Trait definition is not TokenStream");
        let trait_item =
            syn::parse2::<ItemTrait>(stream).expect("Can't parse ItemTrait");

        TraitDefinition {
            0: trait_item,
        }
    }
}

/// Function returns exclusively locked file for metadata.
/// It stores file in the nearest target folder
/// from the directory where the build command has been invoked(output of `pwd` command).
/// If the directory doesn't contain `Cargo.toml` file,
/// it will try to find `Cargo.toml` in the upper directories.
pub(crate) fn get_locked_file() -> File {
    let mut manifest_path =
        PathBuf::from(env::var("PWD").unwrap()).join("Cargo.toml");

    // if the current directory does not contain a Cargo.toml file, go up until you find it.
    while !manifest_path.exists() {
        if let Some(str) = manifest_path.as_os_str().to_str() {
            // If `/Cargo.toml` is not exist, it means that we will do infinity while, so break it
            assert_ne!(str, "/Cargo.toml", "Can't find Cargo.toml in directories tree");
        }
        // Remove Cargo.toml
        manifest_path.pop();
        // Remove parent folder
        manifest_path.pop();
        manifest_path = manifest_path.join("Cargo.toml");
    }

    let mut cmd = MetadataCommand::new();
    let metadata = cmd
        .manifest_path(manifest_path.clone())
        .exec()
        .expect("Error invoking `cargo metadata`");

    let dir = metadata.target_directory.join(TEMP_FILE);

    let file = match OpenOptions::new().read(true).write(true)
        .create(true)
        .open(&dir) {
        Err(why) => panic!("Couldn't open temporary storage: {}", why),
        Ok(file) => file,
    };
    file.lock_exclusive().expect("Can't do exclusive lock");
    file
}

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
            if str.chars().next().unwrap() != '_' {
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
    // Map contains only methods with block section
    let mut trait_methods: HashMap<String, syn::TraitItemMethod> = HashMap::new();
    metadata.external_traits.get(&trait_ident.to_string())
        .methods()
        .into_iter()
        .filter_map(|method|
            if method.default.is_some() {
                Some(method)
            } else {
                None
            }
        )
        .for_each(|method: TraitItemMethod| {
            let key = method.sig.ident.to_string();
            trait_methods.insert(key, method);
        });

    let mut methods_implemented_by_user: HashMap<String, syn::ImplItemMethod> = HashMap::new();
    impl_item.items.clone()
        .into_iter()
        .filter_map(|item|
            if let syn::ImplItem::Method(method) = item {
                Some(method)
            } else {
                None
            }
        )
        .for_each(|method| {
            let key = method.sig.ident.to_string();
            methods_implemented_by_user.insert(key, method);
        });

    impl_item.items
        .iter_mut()
        .for_each(|mut item|
            if let syn::ImplItem::Method(method) = &mut item {
                let method_key = method.sig.ident.to_string();

                let trait_method = trait_methods.get_mut(&method_key).unwrap();
                // Copy attributes from trait definition to user's implementation
                method.attrs.append(&mut trait_method.attrs);

                consume_super_call(method, trait_ident);
            });

    let mut internal_methods: Vec<_> = vec![];
    // Let's create internal `impl section` with default implementation from the trait
    // for method which has been overridden
    let default_impl_of_overridden_methods: Vec<_> = trait_methods.clone()
        .into_iter()
        .filter_map(|(k, v)| {
            if methods_implemented_by_user.contains_key(&k) {
                Some(v)
            } else {
                let item = syn::parse2::<ImplItem>(quote! {
                    #v
                }).unwrap();
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

            method.sig.ident = format_ident!("{}_{}", method.sig.ident, trait_ident);
            let item = syn::parse2::<ImplItem>(quote! {
                #method
            }).unwrap();
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

fn consume_super_call(method: &mut syn::ImplItemMethod, trait_ident: &syn::Ident) {
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
            call.method = format_ident!("{}_{}", call.method, trait_ident);
            call.attrs = call.attrs.clone()
                .into_iter()
                .filter_map(|attr|
                    if attr.path.is_ident("super") {
                        None
                    } else {
                        Some(attr)
                    }
                ).collect();
        }
    });
}

#[inline]
pub(crate) fn is_attr(attrs: &Vec<syn::Attribute>, ident: &str) -> bool {
    if let None = attrs.iter().find(|attr| attr.path.is_ident(ident)) {
        false
    } else {
        true
    }
}