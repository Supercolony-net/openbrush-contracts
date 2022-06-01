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

use proc_macro2::TokenStream;
use quote::{
    format_ident,
    quote,
};
use syn::parse2;

pub fn generate(_: TokenStream, input: TokenStream) -> TokenStream {
    if crate::internal::skip() {
        return (quote! {}).into()
    }
    let mut type_item: syn::ItemType = parse2(input).unwrap();
    if let syn::Type::TraitObject(traits) = &mut *type_item.ty {
        traits.bounds.iter_mut().for_each(|ty| {
            if let syn::TypeParamBound::Trait(t) = ty {
                let trait_ident = t.path.segments.last().expect("Trait path is empty").ident.clone();
                let namespace_ident = format_ident!("{}_external", trait_ident.to_string().to_lowercase());
                t.path
                    .segments
                    .insert(t.path.segments.len() - 1, syn::PathSegment::from(namespace_ident));

                let trait_wrapper_ident = format_ident!("{}Wrapper", trait_ident);
                t.path.segments.pop();
                t.path.segments.push(syn::PathSegment::from(trait_wrapper_ident));
            }
        });

        let mut union_trait = quote! {};
        if traits.bounds.len() > 1 {
            let bounds = traits.bounds.clone();
            let union_ident = traits
                .bounds
                .clone()
                .iter()
                .filter_map(|bound| {
                    if let syn::TypeParamBound::Trait(t) = bound.clone() {
                        Some(t)
                    } else {
                        None
                    }
                })
                .fold(format_ident!("Union"), |acc, t| {
                    format_ident!("{}{}", acc, t.path.segments.last().unwrap().ident)
                });
            traits.bounds.clear();
            let union_bound = syn::TraitBound {
                paren_token: None,
                modifier: syn::TraitBoundModifier::None,
                lifetimes: None,
                path: syn::Path::from(union_ident.clone()),
            };
            traits.bounds.push(syn::TypeParamBound::Trait(union_bound));
            union_trait = quote! {
                pub trait #union_ident : #bounds {}

                impl #union_ident for ::openbrush::traits::AccountId {}
            }
        }

        (quote! {
            #union_trait

            #type_item
        })
        .into()
    } else {
        return (quote! {
                compile_error!(
                    "Macro accepts only form of `type Trait1and2Ref = dyn Trait1 + Trait2`");
        })
        .into()
    }
}
