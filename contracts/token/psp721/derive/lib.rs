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

#[proc_macro_derive(IPSP721)]
pub fn derive_external_psp721(_item: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(_item);
    let code = quote! {
        impl IPSP721 for #ident {}
    };
    code.into()
}

#[proc_macro_derive(IPSP721Metadata)]
pub fn derive_external_psp721_metadata(_item: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(_item);
    let code = quote! {
        impl IPSP721Metadata for #ident {}
    };
    code.into()
}

#[proc_macro_derive(IPSP721Mint)]
pub fn derive_external_psp721_mint(_item: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(_item);
    let code = quote! {
        impl IPSP721Mint for #ident {}
    };
    code.into()
}

#[proc_macro_derive(IPSP721Receiver)]
pub fn derive_external_psp721_receiver(_item: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(_item);
    let code = quote! {
        impl IPSP721Receiver for #ident {}
    };
    code.into()
}