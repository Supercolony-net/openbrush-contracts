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

#[proc_macro_derive(OwnableStorage)]
pub fn derive_ownable_storage(_item: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(_item);
    let code = quote! {
        impl OwnableStorage for #ident {}
    };
    code.into()
}

#[proc_macro_derive(IOwnable)]
pub fn derive_external_ownable(_item: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(_item);
    let code = quote! {
        impl IOwnable for #ident {}
    };
    code.into()
}