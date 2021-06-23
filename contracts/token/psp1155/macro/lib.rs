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

#[proc_macro_derive(PSP1155Storage)]
pub fn derive_psp1155_storage(_item: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(_item);
    let code = quote! {
        impl PSP1155Storage for #ident {}
    };
    code.into()
}

#[proc_macro_derive(IPSP1155)]
pub fn derive_external_psp1155(_item: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(_item);
    let code = quote! {
        impl IPSP1155 for #ident {}
    };
    code.into()
}

#[proc_macro_derive(IPSP1155Metadata)]
pub fn derive_external_psp1155_metadata(_item: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(_item);
    let code = quote! {
        impl IPSP1155Metadata for #ident {}
    };
    code.into()
}

#[proc_macro_derive(IPSP1155Mint)]
pub fn derive_external_psp1155_mint(_item: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(_item);
    let code = quote! {
        impl IPSP1155Mint for #ident {}
    };
    code.into()
}

#[proc_macro_derive(IPSP1155Receiver)]
pub fn derive_external_psp1155_receiver(_item: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(_item);
    let code = quote! {
        impl IPSP1155Receiver for #ident {}
    };
    code.into()
}