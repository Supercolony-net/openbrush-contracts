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
use crate::metadata;

pub(crate) fn generate(_: TokenStream, _input: TokenStream) -> TokenStream {
    let input: TokenStream2 = _input.clone().into();
    let trait_item = parse_macro_input!(_input as ItemTrait);

    let locked_file = metadata::get_locked_file();
    let mut metadata = metadata::Metadata::load(&locked_file);
    metadata.storage_traits.insert(
        trait_item.ident.to_string(), trait_item.into_token_stream().to_string());
    metadata.save_and_unlock(locked_file);

    let code = quote! {
        #input
    };
    code.into()
}