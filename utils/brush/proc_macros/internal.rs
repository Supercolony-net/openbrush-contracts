extern crate proc_macro;

use ink_lang_ir::Callable;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{
    format_ident,
    quote,
};
use std::{
    collections::HashMap,
    convert::TryFrom,
};
use syn::{
    ext::IdentExt,
    parenthesized,
    parse::{
        Parse,
        ParseStream,
    },
    ItemImpl,
};

use crate::{
    metadata::Metadata,
    trait_definition::{
        EXTERNAL_METHOD_SUFFIX,
        EXTERNAL_TRAIT_SUFFIX,
        WRAPPER_TRAIT_SUFFIX,
    },
};

pub(crate) const BRUSH_PREFIX: &'static str = "__brush";

pub(crate) struct MetaList {
    pub path: syn::Path,
    pub _paren_token: syn::token::Paren,
    pub nested: syn::punctuated::Punctuated<TokenStream2, syn::Token![,]>,
}

// Like Path::parse_mod_style but accepts keywords in the path.
fn parse_meta_path(input: ParseStream) -> syn::Result<syn::Path> {
    Ok(syn::Path {
        leading_colon: input.parse()?,
        segments: {
            let mut segments = syn::punctuated::Punctuated::new();
            while input.peek(syn::Ident::peek_any) {
                let ident = syn::Ident::parse_any(input)?;
                segments.push_value(syn::PathSegment::from(ident));
                if !input.peek(syn::Token![::]) {
                    break
                }
                let punct = input.parse()?;
                segments.push_punct(punct);
            }
            if segments.is_empty() {
                return Err(input.error("expected path"))
            } else if segments.trailing_punct() {
                return Err(input.error("expected path segment"))
            }
            segments
        },
    })
}

fn parse_meta_list_after_path(path: syn::Path, input: ParseStream) -> syn::Result<MetaList> {
    let content;
    Ok(MetaList {
        path,
        _paren_token: parenthesized!(content in input),
        nested: content.parse_terminated(TokenStream2::parse)?,
    })
}

fn parse_meta_after_path(path: syn::Path, input: ParseStream) -> syn::Result<NestedMeta> {
    if input.peek(syn::token::Paren) {
        parse_meta_list_after_path(path, input).map(NestedMeta::List)
    } else {
        Ok(NestedMeta::Path(path))
    }
}

impl Parse for MetaList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path = input.call(parse_meta_path)?;
        parse_meta_list_after_path(path, input)
    }
}

pub(crate) enum NestedMeta {
    Path(syn::Path),
    List(MetaList),
}

impl Parse for NestedMeta {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path = input.call(parse_meta_path)?;
        parse_meta_after_path(path, input)
    }
}

pub(crate) struct AttributeArgs(Vec<NestedMeta>);

impl Parse for AttributeArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut attrs = Vec::new();
        while input.peek(syn::Ident::peek_any) {
            attrs.push(input.parse()?);
            if input.is_empty() {
                break;
            }
            let _ : syn::token::Comma = input.parse()?;
        }
        Ok(AttributeArgs { 0: attrs })
    }
}

impl std::ops::Deref for AttributeArgs {
    type Target = Vec<NestedMeta>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for AttributeArgs {
    fn deref_mut(&mut self) -> &mut Vec<NestedMeta> {
        &mut self.0
    }
}

pub(crate) struct Attributes(Vec<syn::Attribute>);

impl Parse for Attributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(syn::Attribute::parse_outer(input)?))
    }
}

impl Attributes {
    pub(crate) fn attr(&self) -> &Vec<syn::Attribute> {
        &self.0
    }
}

// Returns "ink-as-dependency" and not("ink-as-dependency") impls
pub(crate) fn impl_external_trait(
    mut impl_item: syn::ItemImpl,
    trait_ident: &syn::Ident,
    metadata: &Metadata,
) -> (Vec<syn::Item>, Vec<syn::Item>) {
    let impl_ink_attrs = extract_attr(&mut impl_item.attrs, "ink");
    let mut ink_methods: HashMap<String, syn::TraitItemMethod> = HashMap::new();
    metadata
        .external_traits
        .get(&trait_ident.to_string())
        .methods()
        .iter()
        .for_each(|method| {
            if is_attr(&method.attrs, "ink") {
                let mut empty_method = method.clone();
                empty_method.default = Some(
                    syn::parse2(quote! {
                        {
                            unimplemented!()
                        }
                    })
                    .unwrap(),
                );
                let mut attrs = empty_method.attrs.clone();
                empty_method.attrs = extract_attr(&mut attrs, "doc");
                empty_method.attrs.append(&mut extract_attr(&mut attrs, "ink"));
                ink_methods.insert(method.sig.ident.to_string(), empty_method);
            }
        });

    // Move ink! attrs from internal trait to external
    impl_item.items.iter_mut().for_each(|mut item| {
        if let syn::ImplItem::Method(method) = &mut item {
            let method_key = method.sig.ident.to_string();

            if ink_methods.contains_key(&method_key) {
                // Internal attrs will override external, so user must include full declaration with ink(message) and etc.
                ink_methods.get_mut(&method_key).unwrap().attrs = extract_attr(&mut method.attrs, "doc");
                ink_methods
                    .get_mut(&method_key)
                    .unwrap()
                    .attrs
                    .append(&mut extract_attr(&mut method.attrs, "ink"));
            }
        }
    });

    let ink_methods_iter = ink_methods.iter().map(|(_, value)| value);

    let self_ty = impl_item.self_ty.clone().as_ref().clone();
    let draft_impl: ItemImpl = syn::parse2(quote! {
        #(#impl_ink_attrs)*
        impl #trait_ident for #self_ty {
            #(#ink_methods_iter)*
        }
    })
    .unwrap();

    // Evaluate selector and metadata_name for each method based on rules in ink!
    let ink_impl = ::ink_lang_ir::ItemImpl::try_from(draft_impl).unwrap();
    ink_impl.iter_messages().for_each(|message| {
        let method = ink_methods.get_mut(&message.ident().to_string()).unwrap();
        if message.user_provided_selector().is_none() {
            let selector_u32 = u32::from_be_bytes(message.composed_selector().as_bytes().clone()) as usize;
            let selector = format!("{:#010x}", selector_u32);

            method.attrs.push(new_attribute(quote! {#[ink(selector = #selector)]}));
        }
        if message.metadata_name() == message.ident().to_string() {
            let selector = format!("{}", message.metadata_name());

            method
                .attrs
                .push(new_attribute(quote! {#[ink(metadata_name = #selector)]}));
        }

        let original_name = message.ident();
        let inputs_params = message.inputs().map(|pat_type| &pat_type.pat);

        method.default = Some(
            syn::parse2(quote! {
                {
                    #trait_ident::#original_name(self #(, #inputs_params )* )
                }
            })
            .unwrap(),
        );
    });

    let ink_methods_iter = ink_methods.iter().map(|(_, value)| value);
    let wrapper_trait_ident = format_ident!("{}_{}{}", BRUSH_PREFIX, trait_ident, WRAPPER_TRAIT_SUFFIX);
    // We only want to use this implementation in case when ink-as-dependency for wrapper.
    // It will provide methods with the same name like in initial trait.
    let wrapper_impl: ItemImpl = syn::parse2(quote! {
        #(#impl_ink_attrs)*
        impl #wrapper_trait_ident for #self_ty {
            #(#ink_methods_iter)*
        }
    })
    .unwrap();

    let trait_name = ink_impl
        .trait_path()
        .map(|path| path.segments.last().unwrap().ident.to_string());

    let mut metadata_name_attr = quote! {};
    if trait_name == ink_impl.trait_metadata_name() {
        let name = format!("{}", trait_name.unwrap());
        metadata_name_attr = quote! { #[ink(metadata_name = #name)] }
    }
    let external_ink_methods_iter = ink_methods.iter_mut().map(|(_, value)| {
        value.sig.ident = format_ident!("{}_{}{}", BRUSH_PREFIX, value.sig.ident, EXTERNAL_METHOD_SUFFIX);
        value
    });
    let external_trait_ident = format_ident!("{}_{}{}", BRUSH_PREFIX, trait_ident, EXTERNAL_TRAIT_SUFFIX);
    // It is implementation of "external" trait(trait where all method marked with ink!)
    // This trait has another name with external suffix. And all methods have external signature.
    // But ABI generated by this impl section is the same as ABI generated by original trait.
    let external_impl: ItemImpl = syn::parse2(quote! {
        #metadata_name_attr
        #(#impl_ink_attrs)*
        impl #external_trait_ident for #self_ty {
            #(#external_ink_methods_iter)*
        }
    })
    .unwrap();

    // Internal implementation must be disable during "ink-as-dependency"
    let internal_impl = impl_item;

    (
        vec![syn::Item::from(wrapper_impl)],
        vec![syn::Item::from(internal_impl), syn::Item::from(external_impl)],
    )
}

#[inline]
pub(crate) fn is_attr(attrs: &Vec<syn::Attribute>, ident: &str) -> bool {
    if let None = attrs
        .iter()
        .find(|attr| attr.path.segments.last().expect("No segments in path").ident == ident)
    {
        false
    } else {
        true
    }
}

#[inline]
#[allow(dead_code)]
pub(crate) fn get_attr(attrs: &Vec<syn::Attribute>, ident: &str) -> Option<syn::Attribute> {
    for attr in attrs.iter() {
        if is_attr(&vec![attr.clone()], ident) {
            return Some(attr.clone())
        }
    }
    None
}

#[inline]
pub(crate) fn remove_attr(attrs: &Vec<syn::Attribute>, ident: &str) -> Vec<syn::Attribute> {
    attrs
        .clone()
        .into_iter()
        .filter_map(|attr| {
            if is_attr(&vec![attr.clone()], ident) {
                None
            } else {
                Some(attr)
            }
        })
        .collect()
}

#[inline]
pub(crate) fn extract_attr(attrs: &mut Vec<syn::Attribute>, ident: &str) -> Vec<syn::Attribute> {
    attrs.drain_filter(|attr| is_attr(&vec![attr.clone()], ident)).collect()
}

#[inline]
pub(crate) fn new_attribute(attr_stream: TokenStream2) -> syn::Attribute {
    syn::parse2::<Attributes>(attr_stream).unwrap().attr()[0].clone()
}

/// Computes the BLAKE-2b 256-bit hash for the given input and stores it in output.
#[inline]
pub fn blake2b_256(input: &[u8], output: &mut [u8]) {
    use ::blake2::digest::{
        Update as _,
        VariableOutput as _,
    };
    let mut blake2 = blake2::VarBlake2b::new_keyed(&[], 32);
    blake2.update(input);
    blake2.finalize_variable(|result| output.copy_from_slice(result));
}

#[inline]
pub(crate) fn blake2b_256_str(input: String) -> [u8; 32] {
    let mut output: [u8; 32] = [0; 32];
    blake2b_256(&input.into_bytes(), &mut output);
    output
}

#[inline]
pub(crate) fn sanitize_to_str(input: TokenStream) -> String {
    let mut str = input.to_string();
    // Remove quotes rom the string
    str.drain(1..str.len() - 1).collect()
}
