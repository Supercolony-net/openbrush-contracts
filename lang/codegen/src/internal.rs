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

extern crate proc_macro;

use crate::metadata::TraitDefinition;
use heck::CamelCase as _;
use proc_macro2::TokenStream;
use quote::{
    format_ident,
    quote,
};
use std::collections::HashMap;
use syn::{
    ext::IdentExt,
    parenthesized,
    parse::{
        Parse,
        ParseStream,
    },
    ItemImpl,
};

pub(crate) const BRUSH_PREFIX: &'static str = "__openbrush";

pub(crate) struct MetaList {
    pub path: syn::Path,
    pub _paren_token: syn::token::Paren,
    pub nested: syn::punctuated::Punctuated<syn::Expr, syn::Token![,]>,
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
        nested: content.parse_terminated(syn::Expr::parse)?,
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
                break
            }
            let _: syn::token::Comma = input.parse()?;
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

pub(crate) fn impl_external_trait(
    mut impl_item: syn::ItemImpl,
    trait_path: &syn::Path,
    trait_def: &TraitDefinition,
) -> Vec<syn::Item> {
    let trait_ident = trait_path.segments.last().expect("Trait path is empty").ident.clone();
    let namespace_ident = format_ident!("{}_external", trait_ident.to_string().to_lowercase());
    let original_trait_path = trait_path.segments.clone();
    let mut trait_path = trait_path.clone();
    trait_path
        .segments
        .insert(trait_path.segments.len() - 1, syn::PathSegment::from(namespace_ident));
    let impl_ink_attrs = extract_attr(&mut impl_item.attrs, "ink");
    let mut ink_methods: HashMap<String, syn::TraitItemMethod> = HashMap::new();
    trait_def.methods().iter().for_each(|method| {
        if is_attr(&method.attrs, "ink") {
            let mut method = method.clone();

            for (i, fn_arg) in method.sig.inputs.iter_mut().enumerate() {
                if let syn::FnArg::Typed(pat) = fn_arg {
                    let type_ident = format_ident!("{}Input{}", method.sig.ident.to_string().to_camel_case(), i);
                    let mut type_path = trait_path.clone();
                    type_path.segments.pop();
                    type_path.segments.push(syn::PathSegment::from(type_ident));
                    *pat.ty.as_mut() = syn::parse2(quote! {
                        #type_path
                    })
                    .unwrap();
                }
            }

            if let syn::ReturnType::Type(_, t) = &mut method.sig.output {
                let type_ident = format_ident!("{}Output", method.sig.ident.to_string().to_camel_case());
                let mut type_path = trait_path.clone();
                type_path.segments.pop();
                type_path.segments.push(syn::PathSegment::from(type_ident));
                *t = syn::parse2(quote! {
                    #type_path
                })
                .unwrap();
            }

            let original_name = method.sig.ident.clone();
            let inputs_params = method.sig.inputs.iter().filter_map(|fn_arg| {
                if let syn::FnArg::Typed(pat_type) = fn_arg {
                    Some(pat_type.pat.clone())
                } else {
                    None
                }
            });

            method.default = Some(
                syn::parse2(quote! {
                    {
                        #original_trait_path::#original_name(self #(, #inputs_params )* )
                    }
                })
                .unwrap(),
            );
            let mut attrs = method.attrs.clone();
            method.attrs = [extract_attr(&mut attrs, "doc"), extract_attr(&mut attrs, "ink")]
                .into_iter()
                .flatten()
                .collect();
            ink_methods.insert(method.sig.ident.to_string(), method);
        }
    });

    if ink_methods.is_empty() {
        return vec![syn::Item::from(impl_item)]
    }

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

    let self_ty = impl_item.self_ty.as_ref().clone();
    let external_impl: ItemImpl = syn::parse2(quote! {
        #(#impl_ink_attrs)*
        impl #trait_path for #self_ty {
            #(#ink_methods_iter)*
        }
    })
    .unwrap();

    let internal_impl = impl_item;

    vec![
        syn::Item::from(internal_impl.clone()),
        syn::Item::from(external_impl.clone()),
    ]
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
pub(crate) fn new_attribute(attr_stream: TokenStream) -> syn::Attribute {
    syn::parse2::<Attributes>(attr_stream).unwrap().attr()[0].clone()
}

pub(crate) const INK_PREFIX: &str = "ink_lang=";

#[inline]
pub(crate) fn skip() -> bool {
    std::env::args().find(|arg| arg.contains(INK_PREFIX)).is_none()
}
