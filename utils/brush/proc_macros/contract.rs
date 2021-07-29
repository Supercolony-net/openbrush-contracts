use crate::{
    internal,
    internal::*,
    metadata,
    trait_definition,
};
use fs2::FileExt;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{
    quote,
    ToTokens,
};
use syn::Item;

pub(crate) fn generate(_attrs: TokenStream, ink_module: TokenStream) -> TokenStream {
    let input: TokenStream2 = ink_module.into();
    let attrs: TokenStream2 = _attrs.into();
    let mut module = syn::parse2::<syn::ItemMod>(input.clone()).expect("Can't parse contract module");
    let (braces, mut items) = match module.content {
        Some((brace, items)) => (brace, items),
        None => {
            panic!(
                "{}",
                "out-of-line brush modules are not supported, use `#[brush::contract] mod name {{ ... }}`",
            )
        }
    };

    // First, we need to consume all traits and update metadata file.
    // After, we can consume all other stuff.
    items = consume_traits(items);

    let locked_file = metadata::get_locked_file();
    let metadata = metadata::Metadata::load(&locked_file);
    locked_file
        .unlock()
        .expect("Can't remove exclusive lock in extract_fields_and_methods");

    let (ink_items, mut not_ink_items) = split_impls(items, &metadata);

    not_ink_items = add_additional_impls(not_ink_items);

    module.content = Some((braces.clone(), not_ink_items));

    let mut ink_module = module.clone();
    ink_module.content = Some((braces.clone(), ink_items));

    let result = quote! {
        #attrs
        #[cfg(not(feature = "ink-as-dependency"))]
        #[ink_lang::contract]
        #module

        #attrs
        #[cfg(feature = "ink-as-dependency")]
        #[ink_lang::contract]
        #ink_module
    };
    result.into()
}

fn add_additional_impls(mut items: Vec<syn::Item>) -> Vec<syn::Item> {
    let storage = items.iter().find(|item| {
        match item {
            syn::Item::Struct(def) => {
                if is_attr(&def.attrs, "ink") {
                    let attr = get_attr(&def.attrs, "ink").unwrap();
                    attr.tokens.to_string() == "(storage)"
                } else {
                    false
                }
            }
            _ => false,
        }
    });

    if let Some(syn::Item::Struct(def)) = storage {
        let storage_ident = def.ident.clone();

        let item = syn::parse2::<syn::Item>(quote! {
            impl ::brush::traits::Flush for #storage_ident {
                fn flush(&self) {
                    let root_key = ink_primitives::Key::from([0x00; 32]);
                    ink_storage::traits::push_spread_root::<Self>(self, &root_key);
                }
            }
        })
        .unwrap();
        items.push(item);
    }
    items
}

fn consume_traits(items: Vec<syn::Item>) -> Vec<syn::Item> {
    let mut result: Vec<syn::Item> = vec![];
    items.into_iter().for_each(|mut item| {
        if let Item::Trait(item_trait) = &mut item {
            if is_attr(&item_trait.attrs, "trait_definition") {
                item_trait.attrs = remove_attr(&item_trait.attrs, "trait_definition");

                let stream: TokenStream2 =
                    trait_definition::generate(TokenStream::new(), item_trait.to_token_stream().into()).into();
                let mod_item = syn::parse2::<syn::ItemMod>(quote! {
                    mod jora {
                        #stream
                    }
                })
                .expect("Can't parse generated trait definitions");

                let (_, mut generated_items) = mod_item.content.unwrap();
                result.append(&mut generated_items);
            } else {
                result.push(item);
            }
        } else {
            result.push(item);
        }
    });

    result
}

// This function will generate "ink-as-dependency" and not("ink-as-dependency") items.
fn split_impls(mut items: Vec<syn::Item>, metadata: &metadata::Metadata) -> (Vec<syn::Item>, Vec<syn::Item>) {
    let mut ink_items: Vec<syn::Item> = vec![];
    let mut not_ink_items: Vec<syn::Item> = vec![];
    items.iter_mut().for_each(|mut item| {
        if let Item::Impl(item_impl) = &mut item {
            if let Some((_, trait_path, _)) = item_impl.trait_.clone() {
                let trait_ident = trait_path.segments.last().expect("Trait path is empty").ident.clone();
                if metadata.external_traits.contains_key(&trait_ident.to_string()) {
                    let (mut _ink_impls, mut _not_ink_impls) =
                        internal::impl_external_trait(item_impl.clone(), &trait_ident, &metadata);
                    ink_items.append(&mut _ink_impls);
                    not_ink_items.append(&mut _not_ink_impls);
                    return
                }
            }

            // We want to mark all non-external impl sections like ink as dependencies to avoid errors during compilation,
            // because ink! creates wrappers around structures, and impl sections are not valid in this case
            let ink_as_dep_attr = new_attribute(quote! { #[cfg(not(feature = "ink-as-dependency"))] });
            let mut item_impl = item_impl.clone();
            let ink_methods: Vec<_> = item_impl
                .items
                .iter_mut()
                .filter_map(|item| {
                    if let syn::ImplItem::Method(method) = item {
                        if internal::is_attr(&method.attrs, "ink") {
                            Some(method)
                        } else {
                            method.attrs.push(ink_as_dep_attr.clone());
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect();

            if ink_methods.is_empty() {
                item_impl.attrs.push(ink_as_dep_attr);
            }
            ink_items.push(syn::Item::from(item_impl.clone()));
            not_ink_items.push(syn::Item::from(item_impl.clone()));
        } else {
            ink_items.push(item.clone());
            not_ink_items.push(item.clone());
        }
    });

    (ink_items, not_ink_items)
}
