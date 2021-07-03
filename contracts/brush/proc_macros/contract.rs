use crate::{internal, trait_definition};
use quote::{
    quote,
    ToTokens,
};
use syn::{
    Item,
};
use proc_macro::TokenStream;
use proc_macro2::{
    TokenStream as TokenStream2,
    TokenTree,
};
use fs2::FileExt;
use crate::metadata;
use crate::internal::*;
use crate::modifier_definition::{extract_modifier_definitions_impl, extract_modifier_definitions_trait};
use crate::storage_trait;

pub(crate) fn generate(_attrs: TokenStream, ink_module: TokenStream) -> TokenStream {
    let input: TokenStream2 = ink_module.into();
    let attrs: TokenStream2 = _attrs.into();
    let mut module = syn::parse2::<syn::ItemMod>(input.clone()).expect("Can't parse contract module");
    let (braces, mut items) = match module.content {
        Some((brace, items)) => (brace, items),
        None => {
            panic!(
                "{}", "out-of-line ink! modules are not supported, use `#[ink::contract] mod name {{ ... }}`",
            )
        }
    };

    items = consume_modifiers(items);
    // First we need to consume all traits and update metadata file.
    // After we can consume all other stuff.
    items = consume_traits(items);

    let locked_file = metadata::get_locked_file();
    let metadata = metadata::Metadata::load(&locked_file);
    locked_file.unlock().expect("Can't remove exclusive lock in extract_fields_and_methods");

    items = consume_derives(items, &metadata);
    items = consume_impls(items, &metadata);
    module.content = Some((braces, items));

    let result = quote! {
        #attrs
        #[ink_lang::contract]
        #module
    };
    result.into()
}

fn consume_modifiers(items: Vec<syn::Item>) -> Vec<syn::Item> {
    items
        .into_iter()
        .map(|mut item| {
            if let Item::Trait(item_trait) = &mut item {
                extract_modifier_definitions_trait(item_trait);
            } else if let Item::Impl(item_impl) = &mut item {
                extract_modifier_definitions_impl(item_impl);
            }
            item
        }).collect()
}

fn consume_traits(items: Vec<syn::Item>) -> Vec<syn::Item> {
    items
        .into_iter()
        .filter_map(|mut item| {
            if let Item::Trait(item_trait) = &mut item {
                if is_attr(&item_trait.attrs, "storage_trait") {
                    item_trait.attrs = remove_attr(&item_trait.attrs, "storage_trait");

                    let stream = storage_trait::generate(
                        TokenStream::new(), item_trait.to_token_stream().into());
                    let new_trait_item = syn::parse::<syn::Item>(stream)
                        .expect("Can't parse generated storage trait");
                    return Some(new_trait_item)
                } else if is_attr(&item_trait.attrs, "trait_definition") {
                    item_trait.attrs = remove_attr(&item_trait.attrs, "trait_definition");

                    let stream = trait_definition::generate(
                        TokenStream::new(), item_trait.to_token_stream().into());
                    let new_trait_item = syn::parse::<syn::Item>(stream)
                        .expect("Can't parse generated trait definition");
                    return Some(new_trait_item)
                }
            }
            Some(item)
        }).collect()
}

fn consume_impls(mut items: Vec<syn::Item>, metadata: &metadata::Metadata) -> Vec<syn::Item> {
    let mut impls: Vec<TokenStream> = vec![];
    items = items
        .into_iter()
        .filter_map(|mut item| {
            if let Item::Impl(item_impl) = &mut item {
                if let Some((_, trait_path, _)) = item_impl.trait_.clone() {
                    let trait_ident = trait_path.segments
                        .last().expect("Trait path is empty").ident.clone();
                    if metadata.external_traits.contains_key(&trait_ident.to_string()) {
                        let _impl = internal::impl_external_trait(item_impl, &trait_ident, &metadata);
                        impls.push(_impl);
                        return Some(item)
                    }
                }

                // We want to mark all not external impl sections like ink as dependencies to avoid errors during compilation
                // because ink! creates wrappers around structures and impl sections is not valid in this case
                let attr_stream = quote! { #[cfg(not(feature = "ink-as-dependency"))] };
                let ink_as_dep_attr = syn::parse2::<internal::Attributes>(attr_stream).unwrap();

                item_impl.items
                    .iter_mut()
                    .filter_map(|item|
                        if let syn::ImplItem::Method(method) = item {
                            Some(method)
                        } else {
                            None
                        })
                    .for_each(|method| {
                        if !internal::is_attr(&method.attrs, "ink") {
                            method.attrs.append(&mut ink_as_dep_attr.attr().clone());
                        }
                    });
            }
            Some(item)
        }).collect();

    let mut generated_items: Vec<_> = impls
        .into_iter()
        .map(|impl_stream| {
            syn::parse::<syn::ItemImpl>(impl_stream)
                .expect("Can't parse generated implementation")
        })
        .map(|item_impl| syn::Item::from(item_impl))
        .collect();
    items.append(&mut generated_items);
    items
}

fn consume_derives(mut items: Vec<syn::Item>, metadata: &metadata::Metadata) -> Vec<syn::Item> {
    let mut impls: Vec<TokenStream> = vec![];
    items = items
        .into_iter()
        .filter_map(|mut item| {
            if let Item::Struct(item_struct) = &mut item {
                let struct_ident = item_struct.ident.clone();
                let attrs: Vec<syn::Attribute> = item_struct.attrs.clone().iter_mut().map(|attr| {
                    if attr.path.is_ident("derive") {
                        let (fields, mut _impls) =
                            consume_derive(&struct_ident, attr, &metadata);

                        impls.append(&mut _impls);

                        if let syn::Fields::Named(name_fields) = &mut item_struct.fields {
                            fields.into_iter().for_each(|field| name_fields.named.push(field));
                        } else {
                            panic!("Contract support only named fields")
                        }
                    }
                    attr.clone()
                }).collect();
                item_struct.attrs = attrs;
            }
            Some(item)
        }).collect();

    let mut generated_items: Vec<_> = impls
        .into_iter()
        .map(|impl_stream| {
            syn::parse::<syn::ItemImpl>(impl_stream)
                .expect("Can't parse generated implementation")
        })
        .map(|item_impl| syn::Item::from(item_impl))
        .collect();
    items.append(&mut generated_items);
    items
}

fn consume_derive(struct_ident: &syn::Ident,
                  attr: &mut syn::Attribute, metadata: &metadata::Metadata) -> (Vec<syn::Field>, Vec<TokenStream>) {
    let mut fields: Vec<syn::Field> = vec![];
    let mut impls: Vec<TokenStream> = vec![];
    let tokens: TokenStream2 = attr.tokens.clone().into_iter().map(|token|
        if let TokenTree::Group(group) = token {
            let mut last_punct = false;
            let filtered_stream: TokenStream2 = group.stream().into_iter().filter_map(|token|
                if let TokenTree::Punct(_) = token {
                    if last_punct {
                        None
                    } else {
                        last_punct = true;
                        Some(token)
                    }
                } else if let TokenTree::Ident(ident) = &token {
                    let key = ident.to_string();
                    if metadata.storage_traits.contains_key(&key) {
                        let (mut _fields, _impl) = internal::impl_storage_trait(struct_ident, ident, metadata);
                        fields.append(&mut _fields);
                        impls.push(_impl);
                        None
                    } else {
                        last_punct = false;
                        Some(token)
                    }
                } else {
                    last_punct = false;
                    Some(token)
                }
            ).collect();

            proc_macro2::Group::new(group.delimiter(), filtered_stream).into()
        } else {
            token
        }
    ).collect();
    attr.tokens = tokens;
    (fields, impls)
}