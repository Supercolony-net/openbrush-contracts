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

#[proc_macro_derive(PSP17MetadataStorage)]
pub fn derive_psp17metadata_storage(_item: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(_item);
    let code = quote! {
        impl PSP17MetadataStorage for #ident {}
    };
    code.into()
}

#[proc_macro_derive(IPSP20)]
pub fn derive_external_psp20(_item: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(_item);
    let code = quote! {
        impl IPSP20 for #ident {}
    };
    code.into()
}

#[proc_macro_derive(IPSP20Metadata)]
pub fn derive_external_psp20_metadata(_item: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(_item);
    let code = quote! {
        impl IPSP20Metadata for #ident {}
    };
    code.into()
}

#[proc_macro_derive(IPSP20Mint)]
pub fn derive_external_psp20_mint(_item: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(_item);
    let code = quote! {
        impl IPSP20Mint for #ident {}
    };
    code.into()
}

#[proc_macro_derive(IPSP20Receiver)]
pub fn derive_external_psp20_receiver(_item: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(_item);
    let code = quote! {
        impl IPSP20Receiver for #ident {}
    };
    code.into()
}