use quote::{
    quote,
    format_ident,
    ToTokens,
};
use syn::{
    ImplItemMethod,
};
use proc_macro::{TokenStream};
use proc_macro2::{
    TokenStream as TokenStream2,
    TokenTree,
};
use crate::metadata;
use crate::internal::is_attr;

pub(crate) fn generate(_attrs: TokenStream, _input: TokenStream) -> TokenStream {
    let attrs: TokenStream2 = _attrs.into();
    let modifiers: Vec<_> = attrs
        .into_iter()
        .filter_map(|token|
            if let TokenTree::Ident(ident) = token {
                Some(ident)
            } else {
                None
            })
        .collect();

    let mut impl_item = syn::parse2::<ImplItemMethod>(_input.into())
        .expect("Can't parse input of `modifiers` macro like a method.");

    // We skip every function without body(it means that it contains only `{ ; }`)
    if impl_item.block.to_token_stream().to_string() == "{ ; }" {
        let code = quote! {
            #impl_item
        };
        return code.into();
    }

    let locked_file = metadata::get_locked_file();
    let metadata = metadata::Metadata::load(&locked_file);
    metadata.save_and_unlock(locked_file);

    let local_function_ident = format_ident!("{}_local", impl_item.sig.ident);
    let block = impl_item.block.clone();

    // Put the body of original function to local lambda function
    let mut final_stmts = syn::parse2::<syn::Block>(quote! {
        {
            let mut #local_function_ident = || #block;
        }
    }).unwrap().stmts;

    let has_return = impl_item.sig.output != syn::ReturnType::Default;

    let return_ident = format_ident!("{}_out", local_function_ident);
    let function_call;

    if has_return {
        function_call = syn::parse2::<syn::Block>(quote! {
            {
                let #return_ident = #local_function_ident();
            }
        }).unwrap().stmts;
    } else {
        function_call = syn::parse2::<syn::Block>(quote! {
            {
                #local_function_ident();
            }
        }).unwrap().stmts;
    }
    final_stmts.extend(function_call);

    // Code of each modifier must be added in reverse order
    // Code of first modifier {
    //      Code of second modifier {
    //          Code of third modifier {
    //              ...
    //          }
    //      }
    // }
    for modifier_ident in  modifiers.into_iter().rev() {
        let modifier = metadata.modifiers.get(&modifier_ident.to_string());

        let mut local_stmts = vec![];
        for stmt in modifier.block.stmts.clone() {
            if is_body_stmt(&stmt) {
                // We found #[body]() statement, so we put body here
                local_stmts.extend(final_stmts.clone());
                continue;
            }
            local_stmts.push(stmt.clone());
        }
        final_stmts = local_stmts;
    }

    if has_return {
        let return_value = syn::parse2::<syn::Block>(quote! {
            {
                return #return_ident;
            }
        }).unwrap().stmts;
        final_stmts.extend(return_value);
    }

    impl_item.block.stmts = final_stmts;

    let code = quote! {
        #impl_item
    };
    code.into()
}

fn is_body_stmt(stmt: &syn::Stmt) -> bool {
    match stmt {
        syn::Stmt::Semi(expr, _) =>
            match expr {
                syn::Expr::Tuple(tuple) => is_attr(&tuple.attrs, "body"),
                _ => false,
            }
        syn::Stmt::Expr(expr) =>
            match expr {
                syn::Expr::Tuple(tuple) => is_attr(&tuple.attrs, "body"),
                _ => false,
            }
        _ => false,
    }
}