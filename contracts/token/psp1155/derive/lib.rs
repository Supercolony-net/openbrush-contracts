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

#[proc_macro_derive(PSP1155MetadataStorage)]
pub fn derive_psp1155_metadata_storage(_item: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(_item);
    let code = quote! {
        impl PSP1155MetadataStorage for #ident {}
    };
    code.into()
}