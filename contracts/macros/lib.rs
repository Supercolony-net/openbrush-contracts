#![cfg_attr(not(feature = "std"), no_std)]
extern crate proc_macro;

use quote::{
    quote,
    format_ident,
};
use syn::{
    TraitItem,
    ItemTrait,
    Item,
    parse_macro_input,
};
use proc_macro::TokenStream;
use proc_macro2::{
    TokenStream as TokenStream2,
    TokenTree,
};
use std::collections::HashMap;
use std::env;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::str::FromStr;
use serde_json;

const TEMP_FILE: &str = "brush_temp";
type Data = HashMap<String, Vec<String>>;

#[proc_macro_attribute]
pub fn contract(_: TokenStream, ink_module: TokenStream) -> TokenStream {
    let input: TokenStream2 = ink_module.into();
    let mut module = syn::parse2::<syn::ItemMod>(input.clone()).unwrap();
    let (braces, items) = match module.content {
        Some((brace, items)) => (brace, items),
        None => {
            panic!(
                "{}", "out-of-line ink! modules are not supported, use `#[ink::contract] mod name {{ ... }}`",
            )
        }
    };

    let items = items
        .into_iter()
        .map(|item| {
            if let Item::Macro(macro_item) = &item {
                if macro_item.mac.path.is_ident("make_trait") {
                    syn::parse2::<syn::Item>(
                        make_trait(macro_item.mac.tokens.clone().into())
                            .into()
                    ).unwrap()
                } else {
                    item
                }
            } else {
                item
            }
        }).collect();
    module.content = Some((braces, items));

    let result = quote! {
        #[ink_lang::contract]
        #module
    };
    result.into()
}

#[proc_macro]
pub fn make_trait(_item: TokenStream) -> TokenStream {
    // println!("make_trait {}", format!("{:?}", _item));
    let mut tokens: Vec<syn::Ident> = TokenStream2::from(_item)
        .into_iter().filter_map(|item|
        if let TokenTree::Ident(ident) = item {
            Some(ident)
        } else {
            None
        }).collect();

    let contract_ident = tokens.drain(0..1).last().unwrap();

    let contract_traits = tokens;
    let traits = load_hash_map();
    let mut impls: Vec<TokenStream2> = vec![];

    for trait_ident in contract_traits.iter() {
        // println!("{:?}", trait_ident);
        let trait_methods: Vec<_> = traits.get(&trait_ident.to_string()).unwrap().into_iter()
            .map(|method| TokenStream2::from_str(method).unwrap())
            .map(|stream| syn::parse2::<syn::TraitItemMethod>(stream).unwrap())
            .collect();

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
            // transform IErc20 -> Erc20
            let mut chars = trait_ident.to_string().clone();
            chars.remove(0);
            let impl_trait_ident = format_ident!("{}", chars.as_str());

            let output = item.sig.output;
            quote! {
                #( #attrs )*
                fn #ident( #receiver #(, #inputs )* ) #output {
                    <#contract_ident as #impl_trait_ident>::#ident(self #(,  #inputs_params )*)
                }
            }
        });
        let gen = quote! {
            impl #trait_ident for #contract_ident {
                #(#implementations)*
            }
        };
        impls.push(gen.into());
    }

    let impls_iter = impls.iter();
    let final_code = quote! {
        #(#impls_iter)*
    };
    final_code.into()
}

#[proc_macro_attribute]
pub fn reflect_trait(_: TokenStream, input: TokenStream) -> TokenStream {
    // println!("reflect_trait {}", format!("{:?}", input));
    let trait_input = input.clone();
    let trait_item = parse_macro_input!(trait_input as ItemTrait);

    let mut hash_map = load_hash_map();
    put_trait(&mut hash_map, trait_item);
    save_hash_map(hash_map);

    input
}

fn load_hash_map() -> Data {
    let mut dir = env::temp_dir();
    dir = dir.join(TEMP_FILE);

    let file = match OpenOptions::new().read(true).write(true)
        .create(true)
        .open(&dir) {
        Err(why) => panic!("Couldn't open temporary storage: {}", why),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);

    serde_json::from_reader(reader).unwrap_or_default()
}

fn save_hash_map(hashmap: Data) {
    let mut dir = env::temp_dir();
    dir = dir.join(TEMP_FILE);

    println!("Save {}", format!("{:?}", dir));
    let file = OpenOptions::new().write(true).truncate(true).open(&dir).unwrap();
    serde_json::to_writer(file, &hashmap).unwrap();
}

fn put_trait(hash_map: &mut Data, item_trait: ItemTrait) {
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
