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

#[proc_macro_derive(AccessControlStorage)]
pub fn derive_access_control(_item: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(_item);
    let code = quote! {
        impl AccessControlStorage for #ident {}
    };
    code.into()
}

#[proc_macro_derive(IAccessControl)]
pub fn derive_external_access_control(_item: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(_item);
    let code = quote! {
        impl IAccessControl for #ident {}
    };
    code.into()
}