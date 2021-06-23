use crate::internal;
use quote::{
    quote,
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

pub(crate) fn generate(_attrs: TokenStream, ink_module: TokenStream) -> TokenStream {
    let input: TokenStream2 = ink_module.into();
    let attrs: TokenStream2 = _attrs.into();
    let mut module = syn::parse2::<syn::ItemMod>(input.clone()).expect("Can't parse contract module");
    let (braces, items) = match module.content {
        Some((brace, items)) => (brace, items),
        None => {
            panic!(
                "{}", "out-of-line ink! modules are not supported, use `#[ink::contract] mod name {{ ... }}`",
            )
        }
    };
    let locked_file = internal::get_locked_file();
    let metadata = internal::load_metadata(&locked_file);
    locked_file.unlock().expect("Can't remove exclusive lock in extract_fields_and_methods");

    let mut new_items: Vec<syn::Item> = vec![];
    let mut items: Vec<syn::Item> = items
        .into_iter()
        .filter_map(|mut item| {
            if let Item::Struct(item_struct) = &mut item {
                let struct_ident = item_struct.ident.clone();
                let attrs: Vec<syn::Attribute> = item_struct.attrs.clone().iter_mut().map(|attr| {
                    if attr.path.is_ident("derive") {
                        let (fields, impls) =
                            consume_derive(&struct_ident, attr, &metadata);

                        let mut generated_items: Vec<_> = impls
                            .into_iter()
                            .map(|impl_stream| {
                                syn::parse::<syn::ItemImpl>(impl_stream)
                                    .expect("Can't parse generated implementation")
                            })
                            .map(|item_impl| syn::Item::from(item_impl))
                            .collect();

                        new_items.append(&mut generated_items);

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

    items.append(&mut new_items);
    module.content = Some((braces, items));

    let result = quote! {
        #attrs
        #[ink_lang::contract]
        #module
    };
    result.into()
}

fn consume_derive(struct_ident: &syn::Ident,
                  attr: &mut syn::Attribute, metadata: &internal::Metadata) -> (Vec<syn::Field>, Vec<TokenStream>) {
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
                    if metadata.internal_traits.contains_key(&key) {
                        let (mut _fields, _impl) = internal::impl_internal_trait(struct_ident, ident, metadata);
                        fields.append(&mut _fields);
                        impls.push(_impl);
                        None
                    } else if metadata.external_traits.contains_key(&key) {
                        let _impl = internal::impl_external_trait(struct_ident, ident, metadata);
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