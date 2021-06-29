use quote::{
    quote,
    ToTokens,
};
use syn::{
    ItemTrait,
    parse_macro_input,
};
use proc_macro::{TokenStream};
use proc_macro2::{
    TokenStream as TokenStream2,
};
use crate::internal;

pub(crate) fn generate(_attrs: TokenStream, _input: TokenStream) -> TokenStream {
    let input: TokenStream2 = _input.clone().into();
    let attrs: TokenStream2 = _attrs.into();
    let trait_item = parse_macro_input!(_input as ItemTrait);

    let locked_file = internal::get_locked_file();
    let mut metadata = internal::Metadata::load(&locked_file);
    metadata.storage_traits.insert(
        trait_item.ident.to_string(), trait_item.into_token_stream().to_string());
    metadata.save_and_unlock(locked_file);

    let code = quote! {
        #attrs
        #input
    };
    code.into()
}