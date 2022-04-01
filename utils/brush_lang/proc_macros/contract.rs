// Copyright (c) 2012-2022 Supercolony
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

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
    if internal::skip() {
        return (quote! {}).into()
    }
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

    let locked_file = metadata::get_locked_file(crate::metadata::LockType::Shared);
    let metadata = metadata::Metadata::load(&locked_file);
    locked_file.unlock().expect("Can't remove shared lock");

    let generated_items = generate_impls(items, &metadata);

    module.content = Some((braces.clone(), generated_items));

    let result = quote! {
        #[::ink_lang::contract(#attrs)]
        #module
    };
    result.into()
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

fn generate_impls(mut items: Vec<syn::Item>, metadata: &metadata::Metadata) -> Vec<syn::Item> {
    let mut generated_items: Vec<syn::Item> = vec![];
    items.iter_mut().for_each(|mut item| {
        if let Item::Impl(item_impl) = &mut item {
            if let Some((_, trait_path, _)) = item_impl.trait_.clone() {
                let trait_ident = trait_path.segments.last().expect("Trait path is empty").ident.clone();
                if metadata.external_traits.contains_key(&trait_ident.to_string()) {
                    let mut generated_impls = internal::impl_external_trait(item_impl.clone(), &trait_path, &metadata);
                    generated_items.append(&mut generated_impls);
                    return
                }
            }

            generated_items.push(syn::Item::from(item_impl.clone()));
        } else {
            generated_items.push(item.clone());
        }
    });

    generated_items
}
