use crate::{
    internal::{
        is_attr,
        new_attribute,
        remove_attr,
        BRUSH_PREFIX,
    },
    metadata,
};
use heck::CamelCase as _;
use proc_macro::TokenStream;
use quote::{
    format_ident,
    quote,
    ToTokens,
};
use std::collections::HashMap;
use syn::{
    parse_macro_input,
    ItemTrait,
};

pub(crate) fn generate(_attrs: TokenStream, _input: TokenStream) -> TokenStream {
    let attrs: proc_macro2::TokenStream = _attrs.into();
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
    let namespace_ident = format_ident!("{}_{}_{}", BRUSH_PREFIX, "external", trait_item.ident.to_string());

    let mut types: HashMap<syn::Ident, proc_macro2::TokenStream> = HashMap::new();

    ink_trait.items.iter().for_each(|item| {
        if let syn::TraitItem::Method(method) = item {
            if let syn::ReturnType::Type(_, t) = &method.sig.output {
                let type_ident = format_ident!("{}Output", method.sig.ident.to_string().to_camel_case());
                types.insert(type_ident, t.to_token_stream());
            }

            for (i, fn_arg) in method.sig.inputs.iter().enumerate() {
                if let syn::FnArg::Typed(pat) = fn_arg {
                    let type_ident = format_ident!("{}Input{}", method.sig.ident.to_string().to_camel_case(), i);
                    types.insert(type_ident, pat.ty.to_token_stream());
                }
            }
        }
    });

    let aliases = types.iter().map(|(ident, ty)| {
        quote! {
            pub type #ident = #ty;
        }
    });

    let code = quote! {
        // It is original trait defined by user with all features of rust.
        // We removed ink! attributes from methods.
        #trait_without_ink_attrs

        #[allow(non_camel_case_types)]
        pub mod #namespace_ident {
            use super::*;

            #(#aliases)*

            // This trait contains only ink! methods without other attributes.
            #[ink_lang::trait_definition(#attrs)]
            #ink_trait
        }
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
    // Remove all ink attributes form methods
    trait_item.items.iter_mut().for_each(|mut item| {
        if let syn::TraitItem::Method(method) = &mut item {
            method.attrs = remove_attr(&method.attrs, "ink")
        }
    });
    trait_item
}
