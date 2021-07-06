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

#[proc_macro_derive(PSP721Storage)]
pub fn derive_psp721_storage(_item: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(_item);
    let code = quote! {
        impl PSP721Storage for #ident {}
    };
    code.into()
}

#[proc_macro_derive(PSP721MetadataStorage)]
pub fn derive_psp721_metadata_storage(_item: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(_item);
    let code = quote! {
        impl PSP721MetadataStorage for #ident {}
    };
    code.into()
}