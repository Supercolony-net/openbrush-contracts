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

use crate::{
    internal::{
        extract_attr,
        is_attr,
        remove_attr,
    },
    metadata::{
        LockedTrait,
        TraitDefinition,
    },
};
use heck::CamelCase as _;
use proc_macro2::TokenStream;
use quote::{
    format_ident,
    quote,
    ToTokens,
};
use std::collections::HashMap;
use syn::{
    parse2,
    ItemTrait,
};

pub fn generate(_attrs: TokenStream, _input: TokenStream) -> TokenStream {
    if crate::internal::skip() {
        return (quote! {}).into()
    }
    let attrs: proc_macro2::TokenStream = _attrs.into();
    let (mock_type, attrs) = extract_mock_config(attrs);
    let mut trait_item: ItemTrait = parse2(_input).unwrap();
    let trait_without_ink_attrs;
    let ink_code;

    let contains_ink = trait_item.items.iter().find(|item| {
        if let syn::TraitItem::Method(method) = item {
            is_attr(&method.attrs, "ink")
        } else {
            false
        }
    });

    let mut maybe_use_mock_env = quote! {};
    if contains_ink.is_some() {
        add_selectors_attribute(&mut trait_item);
        // Brackets to force the unlock of the file after the update of the trait definition
        {
            // Save trait definition with generics and default methods to metadata.
            let mut trait_lock = LockedTrait::new(trait_item.ident.to_string());
            trait_lock.trait_definition = Some(TraitDefinition::new(trait_item.clone()));
        }

        trait_without_ink_attrs = remove_ink_attrs(trait_item.clone());
        let ink_trait = transform_to_ink_trait(trait_item.clone());
        let namespace_ident = format_ident!("{}_external", trait_item.ident.to_string().to_lowercase());

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

        let wrapper_trait = generate_wrapper(ink_trait.clone(), mock_type.clone());

        ink_code = quote! {
            #[allow(non_camel_case_types)]
            pub mod #namespace_ident {
                use super::*;

                #(#aliases)*

                #wrapper_trait

                // This trait contains only ink! methods without other attributes.
                #[ink_lang::trait_definition(#attrs)]
                #ink_trait
            }
        };

        let pub_mock_env_ident = format_ident!("mock_{}", trait_item.ident.to_string().to_lowercase());
        maybe_use_mock_env = quote! {
            #[cfg(any(test, feature = "mockable"))]
            pub mod #pub_mock_env_ident {
                pub use super :: #namespace_ident :: { mock_env as env , using , deploy };
            }
        };
    } else {
        trait_without_ink_attrs = trait_item;
        ink_code = quote! {};
    }

    let code = quote! {
        // It is original trait defined by user with all features of rust.
        // We removed ink! attributes from methods.
        #trait_without_ink_attrs

        #ink_code

        #maybe_use_mock_env
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
                    method.attrs = extract_attr(&mut method.attrs, "ink");
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

fn generate_wrapper(ink_trait: ItemTrait, mock_type: Option<TokenStream>) -> proc_macro2::TokenStream {
    let trait_ident = ink_trait.ident.clone();
    let trait_wrapper_ident = format_ident!("{}Wrapper", ink_trait.ident);
    let mut def_messages = vec![];
    let mut impl_messages = vec![];
    let mock_address_pattern = name_to_raw_account(&format!("Mock{}", ink_trait.ident));
    ink_trait
        .items
        .clone()
        .into_iter()
        .filter_map(|item| {
            if let syn::TraitItem::Method(method) = item {
                Some(method)
            } else {
                None
            }
        })
        .for_each(|method| {
            let message_ident = method.sig.ident.clone();
            let message_builder_ident = format_ident!("{}_builder", method.sig.ident);
            let output_ty = match method.sig.output.clone() {
                syn::ReturnType::Default => quote! { () },
                syn::ReturnType::Type(_, return_type) => quote! { #return_type },
            };
            let selector_string = format!("{}::{}", trait_ident, message_ident);
            let selector_bytes = ::ink_lang_ir::Selector::compute(&selector_string.into_bytes()).hex_lits();
            let input_bindings = method
                .sig
                .inputs
                .clone()
                .iter()
                .filter_map(|input| {
                    if let syn::FnArg::Typed(pat_typed) = input {
                        Some(pat_typed)
                    } else {
                        None
                    }
                })
                .enumerate()
                .map(|(n, _)| format_ident!("__openbrush_binding_{}", n))
                .collect::<Vec<_>>();
            let input_types = method
                .sig
                .inputs
                .clone()
                .iter()
                .filter_map(|input| {
                    if let syn::FnArg::Typed(pat_typed) = input {
                        Some(pat_typed)
                    } else {
                        None
                    }
                })
                .map(|pat_type| pat_type.ty.clone())
                .collect::<Vec<_>>();
            let arg_list = input_types.iter().cloned().into_iter().fold(
                quote! { ::ink_env::call::utils::EmptyArgumentList },
                |rest, arg| {
                    quote! {
                        ::ink_env::call::utils::ArgumentList<::ink_env::call::utils::Argument<#arg>, #rest>
                    }
                },
            );
            let panic_str = format!(
                "encountered error while calling <AccountId as {}>::{}",
                trait_ident, message_ident,
            );
            def_messages.push(quote! {
                #[inline]
                fn #message_ident(
                    & self
                    #( , #input_bindings : #input_types )*
                ) -> #output_ty;

                #[inline]
                fn #message_builder_ident(
                    & self
                    #( , #input_bindings : #input_types )*
                ) -> ::ink_env::call::CallBuilder<
                    ::ink_env::DefaultEnvironment,
                    ::ink_env::call::utils::Set< ::ink_env::call::Call< ::ink_env::DefaultEnvironment > >,
                    ::ink_env::call::utils::Set< ::ink_env::call::ExecutionInput<#arg_list> >,
                    ::ink_env::call::utils::Set<::ink_env::call::utils::ReturnType<#output_ty>>,
                >;
            });

            let message_test_impl = match &mock_type {
                Some(_mock_ty) => quote! {
                    mock_env :: with(|ctx| {
                        let mut mock_ref = ctx.register.get_mut(self).expect("not an address of mocked contract");
                        ctx.stack.push(&self);
                        let result = mock_ref.borrow_mut(). #message_ident (
                            #( #input_bindings , )*
                        );
                        ctx.stack.pop();
                        result
                    }).expect("mock object not set")
                },
                None => quote! { ::core::panic!("cross-contract call is not supported in ink tests; try to set a mock object?") }
            };

            impl_messages.push(quote! {
                #[inline]
                fn #message_ident(
                    & self
                    #( , #input_bindings : #input_types )*
                ) -> #output_ty {
                    #[cfg(not(any(test, feature = "mockable")))]
                    {
                        Self::#message_builder_ident(self #( , #input_bindings)*)
                            .fire()
                            .unwrap_or_else(|err| ::core::panic!("{}: {:?}", #panic_str, err))
                    }
                    #[cfg(any(test, feature = "mockable"))]
                    {
                        #message_test_impl
                    }
                }

                #[inline]
                fn #message_builder_ident(
                    & self
                    #( , #input_bindings : #input_types )*
                ) -> ::ink_env::call::CallBuilder<
                    ::ink_env::DefaultEnvironment,
                    ::ink_env::call::utils::Set< ::ink_env::call::Call< ::ink_env::DefaultEnvironment > >,
                    ::ink_env::call::utils::Set< ::ink_env::call::ExecutionInput<#arg_list> >,
                    ::ink_env::call::utils::Set<::ink_env::call::utils::ReturnType<#output_ty>>,
                > {
                    ::ink_env::call::build_call::<::ink_env::DefaultEnvironment>()
                        .call_type(
                            ::ink_env::call::Call::new()
                                .callee(self.clone()))
                        .exec_input(
                            ::ink_env::call::ExecutionInput::new(
                                ::ink_env::call::Selector::new([ #( #selector_bytes ),* ])
                            )
                            #(
                                .push_arg(#input_bindings)
                            )*
                        )
                        .returns::<#output_ty>()
                }
            });
        });
    let impl_messages = impl_messages.iter();
    let def_messages = def_messages.iter();

    let maybe_mock_environmental = match mock_type {
        Some(ty) => {
            quote! {
                #[cfg(any(test, feature = "mockable"))]
                pub struct Context {
                    pub stack: ::openbrush::traits::mock::SharedCallStack,
                    pub register: std::collections::BTreeMap<
                        ::openbrush::traits::AccountId,
                        std::rc::Rc<std::cell::RefCell< #ty >>
                    >
                }

                #[cfg(any(test, feature = "mockable"))]
                ::environmental::environmental!(
                    pub mock_env : Context
                );

                #[cfg(any(test, feature = "mockable"))]
                pub fn using<F: FnOnce()>(
                    stack: ::openbrush::traits::mock::SharedCallStack,
                    f: F
                ) {
                    let mut env = Context {
                        stack,
                        register: Default::default()
                    };
                    mock_env::using(&mut env, f);
                }

                #[cfg(any(test, feature = "mockable"))]
                pub fn deploy(inner_contract : #ty) -> (::openbrush::traits::mock::Addressable< #ty >) {
                    let contract: std::rc::Rc<std::cell::RefCell< #ty >> = std::rc::Rc::new(
                        std::cell::RefCell::< #ty >::new(inner_contract)
                    );
                    let (account_id, contract, stack) = mock_env::with(|ctx| {
                        let n: u8 = ctx.register.len().try_into()
                            .expect("too many contracts to fit into u8");
                        let mut pat = [ #( #mock_address_pattern,  )* ];
                        pat[31] = n;
                        let account_id: ::openbrush::traits::AccountId = pat.into();

                        ctx.register.insert(account_id.clone(), contract.clone());
                        (account_id, contract, ctx.stack.clone())
                    }).expect("must call within `using()`");

                    ::openbrush::traits::mock::Addressable::new(
                        account_id,
                        contract,
                        stack,
                    )
                }
            }
        }
        None => quote! {},
    };

    quote! {
        pub trait #trait_wrapper_ident {
            #( #def_messages )*
        }

        impl #trait_wrapper_ident for ::openbrush::traits::AccountId {
            #( #impl_messages )*
        }

        #maybe_mock_environmental
    }
}

fn add_selectors_attribute(trait_item: &mut ItemTrait) {
    let trait_ident = trait_item.ident.clone();
    trait_item.items.iter_mut().for_each(|mut item| {
        if let syn::TraitItem::Method(method) = &mut item {
            if is_attr(&method.attrs, "ink") {
                let contains_selector = method.attrs.iter().find(|attr| {
                    let str_attr = attr.to_token_stream().to_string();
                    str_attr.contains("selector")
                });

                if contains_selector.is_none() {
                    let selector_string = format!("{}::{}", trait_ident, method.sig.ident);
                    let selector_id = ::ink_lang_ir::Selector::compute(&selector_string.into_bytes()).into_be_u32();
                    method.attrs.push(crate::internal::new_attribute(
                        quote! { #[ink(selector = #selector_id)] },
                    ));
                }
            }
        }
    });
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

/// Extracts the mocking related macro args out from the input
///
/// Return a tuple of an optional mock target and the args without the mock target
fn extract_mock_config(attr: TokenStream) -> (Option<TokenStream>, TokenStream) {
    let attr_args = syn::parse2::<attr_args::AttributeArgs>(attr).expect("unable to parse trait_definition attribute");

    let (mock_args, ink_args): (Vec<_>, Vec<_>) = attr_args.into_iter().partition(|arg| arg.name.is_ident("mock"));

    let mock_type = mock_args.first().map(|mock_attr| {
        let ty = &mock_attr.value;
        quote! { #ty }
    });
    let ink_attrs = quote! {
        #( #ink_args , ) *
    };
    (mock_type, ink_attrs)
}

/// Returns a `[u8; 32]` filled with the give str with zero padding.
fn name_to_raw_account(name: &str) -> [u8; 32] {
    let mut v = name.as_bytes().to_vec();
    v.resize(32, 0);
    v.try_into().expect("length is 32; qed.")
}

mod attr_args;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macro_works() {
        let r = generate(
            quote! {
                mock = MyMockType,
                namespace = ::name::space
            },
            quote! {
                pub trait SubmittableOracle {
                    #[ink(message)]
                    fn admin(&self) -> AccountId;

                    #[ink(message)]
                    fn verifier(&self) -> Verifier;

                    #[ink(message)]
                    fn attest(&self, arg: String) -> Result<Attestation, ()>;
                }
            },
        );

        println!("OUTPUT:\n\n{:}", r);
    }
}
