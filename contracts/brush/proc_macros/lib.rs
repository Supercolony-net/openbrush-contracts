extern crate proc_macro;
mod internal;
mod contract;

use quote::{
    quote,
    ToTokens,
};
use syn::{
    ItemTrait,
    ItemFn,
    TraitItemMethod,
    parse_macro_input,
};
use proc_macro::{TokenStream};
use proc_macro2::{
    TokenStream as TokenStream2,
    TokenTree,
};

#[proc_macro_attribute]
pub fn contract(_attrs: TokenStream, ink_module: TokenStream) -> TokenStream {
    contract::generate(_attrs, ink_module)
}

#[proc_macro_attribute]
pub fn trait_definition(_attrs: TokenStream, _input: TokenStream) -> TokenStream {
    let attrs: TokenStream2 = _attrs.into();
    let mut trait_item = parse_macro_input!(_input as ItemTrait);

    // Save trait definition with generics and default methods to metadata.
    let locked_file = internal::get_locked_file();
    let mut metadata = internal::Metadata::load(&locked_file);
    metadata.external_traits.insert(
        trait_item.ident.to_string(), trait_item.clone().into_token_stream().to_string());
    metadata.save_and_unlock(locked_file);

    // ink! doesn't support super trait and default functions, so we need to clean up it
    // Remove super trait
    trait_item.colon_token = None;
    trait_item.supertraits.clear();
    // Remove each default block and add semi colon at the end
    trait_item.items
        .iter_mut()
        .for_each(|item| {
            if let syn::TraitItem::Method(method) = item {
                method.default = None;
                method.semi_token = Some(syn::token::Semi::default());
            }
        });

    // Remove all not ink functions
    trait_item.items = trait_item.items.clone()
        .into_iter()
        .filter_map(|item|
            if let syn::TraitItem::Method(method) = &item {
                if internal::is_attr(&method.attrs, "ink") {
                    Some(item)
                } else {
                    None
                }
            } else {
                Some(item)
            })
        .collect();

    let code = quote! {
        #attrs
        #[ink_lang::trait_definition]
        #trait_item
    };
    code.into()
}

#[proc_macro_attribute]
pub fn storage_trait(_attrs: TokenStream, _input: TokenStream) -> TokenStream {
    let input: TokenStream2 = _input.clone().into();
    let attrs: TokenStream2 = _attrs.into();
    let trait_item = parse_macro_input!(_input as ItemTrait);

    let locked_file = internal::get_locked_file();
    let mut metadata = internal::Metadata::load(&locked_file);
    metadata.storage_traits.insert(
        trait_item.ident.to_string(), trait_item.into_token_stream().to_string());
    metadata.save_and_unlock(locked_file);

    let code = quote! {
        #attrs
        #input
    };
    code.into()
}

#[proc_macro_attribute]
pub fn modifiers(_attrs: TokenStream, method: TokenStream) -> TokenStream {
    let attrs: TokenStream2 = _attrs.into();
    let modifiers = attrs
        .into_iter()
        .filter_map(|token|
            if let TokenTree::Ident(ident) = token {
                Some(ident)
            } else {
                None
            })
        .collect();

    let fn_item = syn::parse2::<ItemFn>(method.clone().into());
    let trait_method_item = syn::parse2::<TraitItemMethod>(method.clone().into());

    let mut code: TokenStream2 = method.into();
    if let Ok(mut item) = fn_item {
        add_modifiers_to_block(&mut item.block, modifiers);
        code = quote! { #item };
    } else if let Ok(mut item) = trait_method_item {
        if let Some(block) = &mut item.default {
            add_modifiers_to_block(block, modifiers);
            code = quote! { #item };
        }
    }

    code.into()
}

#[inline]
fn add_modifiers_to_block(block: &mut syn::Block, modifiers: Vec<syn::Ident>) {
    modifiers
        .into_iter()
        .for_each(|ident| {
            let code = quote! {
                #[cfg(not(feature = "ink-as-dependency"))] self.#ident();
            };
            block.stmts.insert(0, syn::parse2::<syn::Stmt>(code)
                .expect("Can't parse statement of modifier"));
        });
}