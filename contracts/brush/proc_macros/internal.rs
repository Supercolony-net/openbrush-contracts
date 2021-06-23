extern crate proc_macro;

use quote::{
    quote,
    format_ident,
};
use syn::{
    TraitItem,
    ItemTrait,
};
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

const TEMP_FILE: &str = "brush_temp$%$%$";
type Data = HashMap<String, Vec<String>>;

pub(crate) trait Methods {
    fn methods(&self, ident: &String) -> Vec<syn::TraitItemMethod>;
}

impl Methods for Data {
    fn methods(&self, ident: &String) -> Vec<syn::TraitItemMethod> {
        self.get(ident)
            .expect("Can't find definition of trait").into_iter()
            .map(|method| TokenStream2::from_str(method).expect("Can't parse definition of trait"))
            .map(|stream| syn::parse2::<syn::TraitItemMethod>(stream).expect("Can't parse method of trait"))
            .collect()
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub(crate) struct Metadata {
    pub internal_traits: Data,
    pub external_traits: Data,
}

pub(crate) fn get_locked_file() -> File {
    let mut dir = env::temp_dir();
    dir = dir.join(TEMP_FILE);

    let file = match OpenOptions::new().read(true).write(true)
        .create(true)
        .open(&dir) {
        Err(why) => panic!("Couldn't open temporary storage: {}", why),
        Ok(file) => file,
    };
    file.lock_exclusive().expect("Can't do exclusive lock");
    file
}

pub(crate) fn load_metadata(file: &File) -> Metadata {
    let reader = BufReader::new(file);

    let map = serde_json::from_reader(reader).unwrap_or_default();
    map
}

pub(crate) fn save_metadata_and_unlock(mut locked_file: File, metadata: Metadata) {
    locked_file.set_len(0).expect("Can't truncate the file");
    locked_file.seek(SeekFrom::Start(0)).expect("Can't set cursor position");
    serde_json::to_writer(&locked_file, &metadata).expect("Can't dump definition metadata to file");
    locked_file.unlock().expect("Can't remove exclusive lock");
}

pub(crate) fn put_trait(hash_map: &mut Data, item_trait: ItemTrait) {
    let ident = item_trait.ident;
    let items: Vec<_> = item_trait
        .items
        .into_iter()
        .filter_map(|item| {
            if let TraitItem::Method(method) = item {
                Some(quote! { #method })
            } else {
                None
            }
        })
        .map(|x| { x.to_string() })
        .collect();

    hash_map.insert(ident.to_string(), items);
}

// pub struct ImplTrait {
//     pub contract: syn::Ident,
//     pub traits: Vec<(syn::Ident, Option<syn::Ident>)>,
// }
//
// pub fn parse_impl_trait(stream: TokenStream2) -> ImplTrait {
//     let mut iter = stream.into_iter();
//     let mut impl_trait;
//     if let TokenTree::Ident(ident) = iter.next().expect("Empty token stream") {
//         impl_trait = ImplTrait{
//             contract: ident,
//             traits: vec![]
//         };
//     } else {
//         panic!("First token is not struct ident");
//     }
//     for item in iter {
//         if let TokenTree::Punct(_) = item {
//             continue
//         } else if let TokenTree::Ident(trait_ident) = item {
//             impl_trait.traits.push((trait_ident, None));
//         } else if let TokenTree::Group(trait_group) = item {
//             if let Some(pair) = impl_trait.traits.last_mut() {
//                 let internal_trait = &mut pair.1;
//                 if internal_trait.is_none() {
//                     *internal_trait = Some(syn::parse2::<syn::Ident>(trait_group.stream()).expect("Can't find ident of internal trait"));
//                 } else {
//                     panic!("External trait already contains internal");
//                 }
//             } else {
//                 panic!("Internal trait before external");
//             }
//         }
//     }
//     impl_trait
// }

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

pub(crate) fn impl_internal_trait(struct_ident: &syn::Ident, trait_ident: &syn::Ident, metadata: &Metadata) -> (Vec<syn::Field>, TokenStream) {
    let trait_methods = metadata.internal_traits.methods(&trait_ident.to_string());

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

pub(crate) fn impl_external_trait(struct_ident: &syn::Ident, trait_ident: &syn::Ident, metadata: &Metadata) -> TokenStream {
    let trait_methods = metadata.external_traits.methods(&trait_ident.to_string());

    let implementations = trait_methods.into_iter().map(|item| {
        let attrs = item.attrs;
        let ident = item.sig.ident;
        let inputs = item.sig.inputs.iter().skip(1);
        let inputs_params = item.sig.inputs.iter()
            .skip(1)
            .filter_map(|pat_type| {
                if let syn::FnArg::Typed(pat) = pat_type {
                    let pat_ident = &pat.pat;
                    Some(quote! { #pat_ident })
                } else {
                    None
                }
            });
        let receiver = match item.sig.inputs.iter().next() {
            Some(syn::FnArg::Receiver(receiver)) => {
                debug_assert!(receiver.reference.is_some());
                if receiver.mutability.is_some() {
                    quote! { &mut self }
                } else {
                    quote! { &self }
                }
            }
            _ => unreachable!("encountered invalid receiver argument for brush message"),
        };
        // transform IPSP20 -> PSP20
        let mut chars = trait_ident.to_string().clone();
        chars.remove(0);
        let internal_trait = format_ident!("{}", chars.as_str());

        let output = item.sig.output;
        quote! {
            #( #attrs )*
            fn #ident( #receiver #(, #inputs )* ) #output {
                <#struct_ident as #internal_trait>::#ident(self #(,  #inputs_params )*)
            }
        }
    });
    let gen = quote! {
        impl #trait_ident for #struct_ident {
            #(#implementations)*
        }
    };
    gen.into()
}