use crate::{
    internal::{
        is_attr,
        new_attribute,
        remove_attr,
        BRUSH_PREFIX,
    },
    metadata,
};
use proc_macro::TokenStream;
use quote::{
    format_ident,
    quote,
    ToTokens,
};
use syn::{
    parse_macro_input,
    ItemTrait,
};

pub(crate) const WRAPPER_TRAIT_SUFFIX: &'static str = "Wrapper";
pub(crate) const EXTERNAL_TRAIT_SUFFIX: &'static str = "External";
pub(crate) const EXTERNAL_METHOD_SUFFIX: &'static str = "_external";

pub(crate) fn generate(_: TokenStream, _input: TokenStream) -> TokenStream {
    let trait_item = parse_macro_input!(_input as ItemTrait);

    // Save trait definition with generics and default methods to metadata.
    let locked_file = metadata::get_locked_file();
    let mut metadata = metadata::Metadata::load(&locked_file);
    metadata.external_traits.insert(
        trait_item.ident.to_string(),
        trait_item.clone().into_token_stream().to_string(),
    );
    metadata.save_and_unlock(locked_file);

    let trait_without_ink_attrs = remove_ink_attrs(trait_item.clone());
    let ink_trait = transform_to_ink_trait(trait_item.clone());
    let mut ink_wrapper = ink_trait.clone();
    ink_wrapper.ident = format_ident!("{}_{}{}", BRUSH_PREFIX, ink_wrapper.ident, WRAPPER_TRAIT_SUFFIX);

    // Create external trait with external method. This trait will call implementation of internal trait.
    // During implementation of this trait we will
    let mut ink_external = ink_trait;
    ink_external.ident = format_ident!("{}_{}{}", BRUSH_PREFIX, ink_external.ident, EXTERNAL_TRAIT_SUFFIX);
    ink_external.items.iter_mut().for_each(|item| {
        if let syn::TraitItem::Method(method) = item {
            method.sig.ident = format_ident!("{}_{}{}", BRUSH_PREFIX, method.sig.ident, EXTERNAL_METHOD_SUFFIX)
        }
    });

    let code = quote! {
        // It is original trait defined by user with all features of rust.
        // We removed ink! attributes from methods.
        #trait_without_ink_attrs

        // This trait contains only ink! methods with modified name.
        // This trait will use metadata_name and selector attributes
        // to generate the same ABI like original trait.
        #[ink_lang::trait_definition]
        #[allow(non_camel_case_types)]
        #ink_external

        // This trait contains only ink! methods with original naming.
        // We will use them to cover "ink-as-dependency" case.
        // We will implement this trait only for this case.
        #[ink_lang::trait_definition]
        #[allow(non_camel_case_types)]
        #ink_wrapper
    };
    code.into()
}

fn transform_to_ink_trait(mut trait_item: ItemTrait) -> ItemTrait {
    // ink! doesn't support super traits, generics, not ink! methods and default functions, so we need to clean it up
    // Remove super trait
    trait_item.colon_token = None;
    trait_item.generics.lt_token = None;
    trait_item.generics.params.clear();
    trait_item.generics.gt_token = None;
    trait_item.generics.where_clause = None;
    trait_item.supertraits.clear();
    // Remove each default block and add semi colon at the end
    trait_item.items.iter_mut().for_each(|item| {
        if let syn::TraitItem::Method(method) = item {
            method.default = None;
            method.semi_token = Some(syn::token::Semi::default());
        }
    });

    // Remove all non-ink functions
    trait_item.items = trait_item
        .items
        .clone()
        .into_iter()
        .filter_map(|mut item| {
            if let syn::TraitItem::Method(method) = &mut item {
                if is_attr(&method.attrs, "ink") {
                    // Remove every attribute except `#[ink(message)]` and `#[ink(constructor)]`
                    // Because ink! doesn't allow another attributes in the trait
                    // We will paste that attributes back in impl section
                    method.attrs = method
                        .attrs
                        .clone()
                        .into_iter()
                        .filter_map(|attr| {
                            let str_attr = attr.to_token_stream().to_string();

                            if str_attr.contains("#[ink") {
                                if str_attr.contains("message") {
                                    Some(new_attribute(quote! { #[ink(message)] }))
                                } else if str_attr.contains("constructor") {
                                    Some(new_attribute(quote! { #[ink(constructor)] }))
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        })
                        .collect();
                    Some(item)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    trait_item
}

fn remove_ink_attrs(mut trait_item: ItemTrait) -> ItemTrait {
    // Remove all non-ink functions
    trait_item.items.iter_mut().for_each(|mut item| {
        if let syn::TraitItem::Method(method) = &mut item {
            method.attrs = remove_attr(&method.attrs, "ink")
        }
    });
    trait_item
}
