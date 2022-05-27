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
    quote,
    quote_spanned,
    ToTokens,
};
use syn::{
    parse2,
    spanned::Spanned,
    ItemFn,
};

pub fn generate(_: TokenStream, _input: TokenStream) -> TokenStream {
    let fn_item: ItemFn = parse2(_input).unwrap();

    if fn_item.sig.inputs.len() < 2 {
        return (quote_spanned! {
            fn_item.sig.inputs.span() =>
                compile_error!(
                    "Modifier must take at least two arguments, \
                    where first is a reference to instance `instance: \
                    & Trait/Struct` and second is Fn, FnMut or FnOnce");
        })
        .into()
    }

    let instance_ty: syn::TypeReference;
    let first = fn_item.sig.inputs.first().unwrap();
    if let syn::FnArg::Typed(pat) = first {
        if let syn::Type::Reference(refer) = pat.ty.as_ref() {
            instance_ty = refer.clone();
        } else {
            return (quote_spanned! {
                pat.ty.as_ref().span() =>
                    compile_error!("First argument of modifier must be a reference to instance `&T` or `&mut T`");
            })
            .into()
        }
    } else {
        return (quote_spanned! {
            first.span() =>
                compile_error!("First argument of modifier can't be `self`");
        })
        .into()
    }

    let return_ty = fn_item.sig.output.clone();
    let mut fn_string = format!(
        "Fn({}) {}",
        instance_ty.to_token_stream().to_string(),
        return_ty.to_token_stream().to_string()
    );

    let mut fn_mut_string = format!(
        "FnMut({}) {}",
        instance_ty.to_token_stream().to_string(),
        return_ty.to_token_stream().to_string()
    );

    let mut fn_once_string = format!(
        "FnOnce({}) {}",
        instance_ty.to_token_stream().to_string(),
        return_ty.to_token_stream().to_string()
    );
    let err_message = format!(
        "Second argument of modifier must be body `{}`, `{}` or `{}`",
        fn_string.as_str(),
        fn_mut_string.as_str(),
        fn_once_string.as_str()
    );

    fn_string.retain(|c| !c.is_whitespace());
    fn_mut_string.retain(|c| !c.is_whitespace());
    fn_once_string.retain(|c| !c.is_whitespace());

    let second = fn_item.sig.inputs.iter().skip(1).next().unwrap();
    if let syn::FnArg::Typed(pat) = second {
        let mut found = false;
        let mut found_ty = None;
        let mut found_span = None;
        let mut t = pat.ty.to_token_stream().to_string();
        t.retain(|c| !c.is_whitespace());
        if t.contains(&fn_string) || t.contains(&fn_mut_string) || t.contains(&fn_once_string) {
            found_ty = Some(t.clone());
            found_span = Some(pat.ty.span().clone());
            found = true;
        }

        let generic = fn_item
            .sig
            .generics
            .params
            .iter()
            .filter_map(|param| {
                if let syn::GenericParam::Type(type_param) = &param {
                    Some(type_param)
                } else {
                    None
                }
            })
            .find(|type_param| type_param.ident.to_string() == t);

        if let Some(generic) = generic {
            if let Some(generic_bound) = &generic.bounds.first() {
                let mut t = generic_bound.to_token_stream().to_string();
                t.retain(|c| !c.is_whitespace());
                if t.contains(&fn_string) || t.contains(&fn_mut_string) || t.contains(&fn_once_string) {
                    found_ty = Some(t);
                    found_span = Some(generic_bound.span().clone());
                    found = true;
                }
            }
        }

        if let Some(where_clause) = &fn_item.sig.generics.where_clause {
            let predicate = where_clause
                .predicates
                .iter()
                .filter_map(|pred| {
                    if let syn::WherePredicate::Type(type_pred) = &pred {
                        Some(type_pred)
                    } else {
                        None
                    }
                })
                .find(|type_pred| type_pred.bounded_ty.to_token_stream().to_string() == t);

            if let Some(pred) = predicate {
                if let Some(pred_bound) = pred.bounds.first() {
                    let mut t = pred_bound.to_token_stream().to_string();
                    t.retain(|c| !c.is_whitespace());
                    if t.contains(&fn_string) || t.contains(&fn_mut_string) || t.contains(&fn_once_string) {
                        found_ty = Some(t);
                        found_span = Some(pred_bound.span().clone());
                        found = true;
                    }
                }
            }
        }

        if !found {
            return (quote_spanned! {
                pat.ty.span() =>
                    compile_error!(#err_message);
            })
            .into()
        } else {
            let mut modifier_ty_str = fn_item.sig.output.to_token_stream().to_string();
            modifier_ty_str.retain(|c| !c.is_whitespace());
            let found_ty = found_ty.unwrap();

            let found_index = found_ty.find("->").unwrap_or(found_ty.len());
            let found_ty_str = &found_ty[found_index..];

            if found_ty_str != modifier_ty_str {
                return (quote_spanned! {
                    found_span.unwrap().span() =>
                        compile_error!("Return type of body mismatched with return type of modifier");
                })
                .into()
            }
        }
    } else if let syn::FnArg::Receiver(rec) = first {
        return (quote_spanned! {
            rec.span() =>
                compile_error!("Second argument of modifier can't be `self`");
        })
        .into()
    }

    for arg in fn_item.sig.inputs.iter().skip(2) {
        if let syn::FnArg::Typed(arg) = arg {
            if let syn::Type::Reference(refer) = arg.ty.as_ref() {
                return (quote_spanned! {
                    refer.span() =>
                        compile_error!("The argument is a reference. \
                        Modifier only accepts arguments which implement `Clone` trait and only by value.");
                })
                .into()
            }
        } else {
            return (quote_spanned! {
                arg.span() =>
                    compile_error!("`self` is not allowed.");
            })
            .into()
        }
    }

    let code = quote! {
        #fn_item
    };
    code.into()
}
