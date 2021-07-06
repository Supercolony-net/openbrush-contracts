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

#[proc_macro_derive(PSP20Storage)]
pub fn derive_psp20_storage(_item: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(_item);
    let code = quote! {
        impl PSP20Storage for #ident {}
    };
    code.into()
}

#[proc_macro_derive(PSP20MetadataStorage)]
pub fn derive_psp20metadata_storage(_item: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(_item);
    let code = quote! {
        impl PSP20MetadataStorage for #ident {}
    };
    code.into()
}