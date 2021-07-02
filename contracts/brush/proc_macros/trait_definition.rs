use quote::{
    quote,
    ToTokens,
};
use syn::{
    ItemTrait,
    parse_macro_input,
};
use proc_macro::{TokenStream};
use crate::internal::{is_attr};
use crate::metadata;

pub(crate) fn generate(_: TokenStream, _input: TokenStream) -> TokenStream {
    let trait_item = parse_macro_input!(_input as ItemTrait);

    // Save trait definition with generics and default methods to metadata.
    let locked_file = metadata::get_locked_file();
    let mut metadata = metadata::Metadata::load(&locked_file);
    metadata.external_traits.insert(
        trait_item.ident.to_string(), trait_item.clone().into_token_stream().to_string());
    metadata.save_and_unlock(locked_file);

    let ink_trait = transform_to_ink_trait(trait_item);
    let code = quote! {
        #[ink_lang::trait_definition]
        #ink_trait
    };
    code.into()
}

fn transform_to_ink_trait(mut trait_item: ItemTrait) -> ItemTrait {
    // ink! doesn't support super traits and default functions, so we need to clean it up
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
                if is_attr(&method.attrs, "ink") {
                    Some(item)
                } else {
                    None
                }
            } else {
                Some(item)
            })
        .collect();
    trait_item
}