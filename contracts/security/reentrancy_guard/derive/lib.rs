extern crate proc_macro;
use proc_macro::{
    TokenStream,
};
use quote::{
    quote,
};
use syn::{
    DeriveInput,
    parse_macro_input,
};

#[proc_macro_derive(ReentrancyGuardStorage)]
pub fn derive_ownable_storage(_item: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(_item);
    let code = quote! {
        impl ReentrancyGuardStorage for #ident {}
    };
    code.into()
}