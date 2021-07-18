use quote::{
    quote,
    quote_spanned,
    format_ident,
    ToTokens,
};
use syn::{
    ImplItemMethod,
    parse_macro_input,
    spanned::Spanned,
};
use proc_macro::{TokenStream};
use proc_macro2::{
    TokenStream as TokenStream2,
    TokenTree,
};

const INSTANCE: &'static str = "__brush_instance_modifier";

pub(crate) fn generate(_attrs: TokenStream, _input: TokenStream) -> TokenStream {
    let input: TokenStream2 = _input.clone().into();

    let modifiers = parse_macro_input!(_attrs as syn::AttributeArgs);
    let mut impl_item = syn::parse2::<ImplItemMethod>(_input.into())
        .expect("Can't parse input of `modifiers` macro like a method.");

    if impl_item.sig.inputs.is_empty() {
        return (quote_spanned! {
            impl_item.sig.inputs.span() =>
                compile_error!("Modifiers can only be applied to method whose first argument is `self`. ");
        }).into();
    }

    let receiver;
    if let syn::FnArg::Receiver(rec) = impl_item.sig.inputs.first().unwrap() {
        receiver = rec;
    } else {
        return (quote_spanned! {
            impl_item.sig.inputs.first().unwrap().span() =>
                compile_error!("First argument in modifiers must be `self`.");
        }).into();
    }

    // We skip every function without body(it means that it contains only `{ ; }`)
    if impl_item.block.to_token_stream().to_string() == "{ ; }" {
        let code = quote! {
            #impl_item
        };
        return code.into();
    }

    let mut block = impl_item.block.clone();
    let mut body_index = 0;

    let modifiers: Vec<_> = modifiers.into_iter().filter_map(|nested_meta| {
        match nested_meta {
            syn::NestedMeta::Meta(meta) => Some(meta),
            _ => None,
        }
    }).collect();

    // Code of each modifier must be added in reverse order
    // Code of first modifier {
    //      Code of second modifier {
    //          Code of third modifier {
    //              ...
    //          }
    //      }
    // }
    for modifier_meta in  modifiers.into_iter().rev() {
        // Replace every `self` with instance variable
        block = replace_self(block);

        // Put the body of original function to local lambda function
        let (final_block, body_ident) = put_into_closure(receiver, block, body_index);
        body_index += 1;

        // It means modifiers without arguments, we can call path method directly.
        if let syn::Meta::Path(method) = modifier_meta {
            let stmts = final_block.stmts;
            block = syn::parse2::<syn::Block>(quote! {
                {
                    #(#stmts)*
                    #method(self, #body_ident)
                }
            }).unwrap();
        } else if let syn::Meta::List(meta_list) = modifier_meta {
            let method = meta_list.path;
            let mut cloned_variables_idents = vec![];
            let cloned_variables_definitions = meta_list.nested.iter()
                .map(|nested_meta| {
                    let cloned_ident = format_ident!("__brush_cloned_{}", cloned_variables_idents.len());
                    cloned_variables_idents.push(cloned_ident.clone());
                    quote! {
                        let #cloned_ident = #nested_meta.clone();
                    }
                });

            let stmts = final_block.stmts;
            block = syn::parse2::<syn::Block>(quote! {
                {
                    #(#cloned_variables_definitions)*
                    #(#stmts)*
                    #method(self, #body_ident #(, #cloned_variables_idents )*)
                }
            }).unwrap();
        } else {
            return (quote_spanned! {
                modifier_meta.span() =>
                    compile_error!("Modifiers doesn't support MetaNameValue in arguments");
            }).into();
        }
    }

    impl_item.block = block;

    let code = quote! {
        #[cfg(not(feature = "ink-as-dependency"))]
        #impl_item

        #[cfg(feature = "ink-as-dependency")]
        #input
    };

    code.into()
}

fn replace_self(block: syn::Block) -> syn::Block {
    syn::parse2::<syn::Block>(recursive_replace_self(block.to_token_stream())).unwrap()
}

fn recursive_replace_self(token_stream: TokenStream2) -> TokenStream2 {
    token_stream.into_iter()
        .map(|token| {
            match &token {
                TokenTree::Ident(ident) =>
                    if ident.to_string() == "self" {
                        TokenTree::Ident(format_ident!("{}", INSTANCE))
                    } else {
                        token
                    },
                TokenTree::Group(group) => {
                    TokenTree::Group(
                        proc_macro2::Group::new(
                            group.delimiter(),
                            recursive_replace_self(group.stream())
                        )
                    )
                }
                _ => token,
            }
        }).collect()
}

fn put_into_closure(receiver: &syn::Receiver, block: syn::Block, index: u8) -> (syn::Block, syn::Ident) {
    let body_ident = format_ident!("__brush_body_{}", index);
    let instance_ident = format_ident!("{}", INSTANCE);

    let reference = match receiver.mutability.is_some() {
        true => quote! { &mut },
        false => quote! { & },
    };

    // Put the body of original function to local lambda function
    let final_block = syn::parse2::<syn::Block>(quote! {
        {
            let mut #body_ident = |#instance_ident: #reference Self| #block;
        }
    }).unwrap();

    (final_block, body_ident)
}
