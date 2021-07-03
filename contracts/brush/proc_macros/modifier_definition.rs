use quote::{
    ToTokens,
};
use syn::{
    ImplItemMethod,
    parse_macro_input,
};
use proc_macro::{TokenStream};
use crate::metadata;
use crate::internal::is_attr;

pub(crate) fn generate(_: TokenStream, _input: TokenStream) -> TokenStream {
    let mut impl_item = parse_macro_input!(_input as ImplItemMethod);

    let locked_file = metadata::get_locked_file();
    let mut metadata = metadata::Metadata::load(&locked_file);
    metadata.modifiers.insert(
        impl_item.sig.ident.to_string(), impl_item.clone().into_token_stream().to_string());
    metadata.save_and_unlock(locked_file);

    let bodies: Vec<_> = impl_item.block.stmts
        .iter_mut()
        .filter_map(|stmt|
            if let syn::Stmt::Expr(exp) = stmt {
                Some(exp)
            } else if let syn::Stmt::Semi(exp, _) = stmt {
                Some(exp)
            } else {
                None
            })
        .filter_map(|expr|
            if let syn::Expr::Tuple(tuple) = expr {
                if is_attr(&tuple.attrs, "body") {
                    Some(tuple)
                } else {
                    None
                }
            } else {
                None
            })
        .collect();

    assert!(!bodies.is_empty(), "Modifier must contains definition of body to place.");
    assert_eq!(bodies.len(), 1, "Modifier contains more than one place for body. Must be only one");

    TokenStream::new()
}

pub(crate) fn extract_modifier_definitions_trait(trait_item: &mut syn::ItemTrait) {
    trait_item.items = trait_item.items.clone()
        .into_iter()
        .filter_map(|item|
            if let syn::TraitItem::Method(method) = &item {
                if is_attr(&method.attrs, "modifier_definition") {
                    generate(TokenStream::new(), method.to_token_stream().into());
                    None
                } else {
                    Some(item)
                }
            } else {
                Some(item)
            })
        .collect();
}

pub(crate) fn extract_modifier_definitions_impl(impl_item: &mut syn::ItemImpl) {
    impl_item.items = impl_item.items.clone()
        .into_iter()
        .filter_map(|item|
            if let syn::ImplItem::Method(method) = &item {
                if is_attr(&method.attrs, "modifier_definition") {
                    generate(TokenStream::new(), method.to_token_stream().into());
                    None
                } else {
                    Some(item)
                }
            } else {
                Some(item)
            })
        .collect();
}