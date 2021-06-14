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
    parse::Parser,
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
use fs2::FileExt;

const TEMP_FILE: &str = "brush_temp$%$%$";
type Data = HashMap<String, Vec<String>>;

#[proc_macro_attribute]
pub fn contract(_attrs: TokenStream, ink_module: TokenStream) -> TokenStream {
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

    let mut new_items: Vec<syn::Item> = vec![];
    let mut items: Vec<syn::Item> = items
        .into_iter()
        .filter_map(|item| {
            if let Item::Macro(macro_item) = &item {
                if macro_item.mac.path.is_ident("impl_trait") {
                    let mut generated_items = syn::Block::parse_within.
                        parse(impl_trait(macro_item.mac.tokens.clone().into()))
                        .expect("Can't parse generate impl code")
                        .iter_mut()
                        .filter_map(|stmt|
                            if let syn::Stmt::Item(item) = stmt {
                                Some(item.clone())
                            } else {
                                None
                            }
                        ).collect();
                    new_items.append(&mut generated_items);
                    None
                } else {
                    Some(item)
                }
            } else {
                Some(item)
            }
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

#[proc_macro_attribute]
pub fn trait_definition(_attrs: TokenStream, _input: TokenStream) -> TokenStream {
    let input: TokenStream2 = _input.clone().into();
    let attrs: TokenStream2 = _attrs.into();
    let trait_item = parse_macro_input!(_input as ItemTrait);

    let mut hash_map = load_hash_map();
    put_trait(&mut hash_map, trait_item);
    save_hash_map(hash_map);

    let code = quote! {
        #attrs
        #[ink_lang::trait_definition]
        #input
    };
    code.into()
}

struct ImplTrait {
    contract: syn::Ident,
    traits: Vec<(syn::Ident, Option<syn::Ident>)>,
}

fn parse_impl_trait(stream: TokenStream2) -> ImplTrait {
    let mut iter = stream.into_iter();
    let mut impl_trait;
    if let TokenTree::Ident(ident) = iter.next().expect("Empty token stream") {
        impl_trait = ImplTrait{
            contract: ident,
            traits: vec![]
        };
    } else {
        panic!("First token is not struct ident");
    }
    for item in iter {
        if let TokenTree::Punct(_) = item {
            continue
        } else if let TokenTree::Ident(trait_ident) = item {
            impl_trait.traits.push((trait_ident, None));
        } else if let TokenTree::Group(trait_group) = item {
            if let Some(pair) = impl_trait.traits.last_mut() {
                let internal_trait = &mut pair.1;
                if internal_trait.is_none() {
                    *internal_trait = Some(syn::parse2::<syn::Ident>(trait_group.stream()).expect("Can't find ident of internal trait"));
                } else {
                    panic!("External trait already contains internal");
                }
            } else {
                panic!("Internal trait before external");
            }
        }
    }
    impl_trait
}

#[proc_macro]
pub fn impl_trait(_item: TokenStream) -> TokenStream {
    let impl_trait = parse_impl_trait(TokenStream2::from(_item));

    let contract_ident = impl_trait.contract;

    let traits = load_hash_map();
    let mut impls: Vec<TokenStream2> = vec![];

    // println!("{:?}", impl_trait.clone());
    for (external_trait, internal) in impl_trait.traits.iter() {
        let trait_methods: Vec<_> = traits.get(&external_trait.to_string())
            .expect("Can't find definition of external trait").into_iter()
            .map(|method| TokenStream2::from_str(method).expect("Can't parse definition of external trait"))
            .map(|stream| syn::parse2::<syn::TraitItemMethod>(stream).expect("Can't parse external method"))
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
            let internal_trait;
            if let Some(t) = internal {
                internal_trait = t.clone();
            } else {
                // transform IErc20 -> Erc20
                let mut chars = external_trait.to_string().clone();
                chars.remove(0);
                internal_trait = format_ident!("{}", chars.as_str());
            }

            let output = item.sig.output;
            quote! {
                #( #attrs )*
                fn #ident( #receiver #(, #inputs )* ) #output {
                    <#contract_ident as #internal_trait>::#ident(self #(,  #inputs_params )*)
                }
            }
        });
        let gen = quote! {
            impl #external_trait for #contract_ident {
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

fn load_hash_map() -> Data {
    let mut dir = env::temp_dir();
    dir = dir.join(TEMP_FILE);

    let file = match OpenOptions::new().read(true).write(true)
        .create(true)
        .open(&dir) {
        Err(why) => panic!("Couldn't open temporary storage: {}", why),
        Ok(file) => file,
    };
    file.lock_shared().expect("Can't do shared lock");
    let reader = BufReader::new(&file);

    let map = serde_json::from_reader(reader).unwrap_or_default();
    file.unlock().expect("Can't remove shared lock");
    map
}

fn save_hash_map(hashmap: Data) {
    let mut dir = env::temp_dir();
    dir = dir.join(TEMP_FILE);

    let mut file = OpenOptions::new().write(true).truncate(true).open(&dir)
        .expect("Can't open file with truncation");
    file.lock_exclusive().expect("Can't do exclusive lock");
    serde_json::to_writer(&mut file, &hashmap).expect("Can't dump definition map to file");
    file.unlock().expect("Can't remove exclusive lock");
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
