#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp34_metadata {
    use openbrush::contracts::psp34::extensions::metadata::*;
    use ink_prelude::{
        string::String,
        vec::Vec,
    };
    use ink_storage::traits::SpreadAllocate;

    #[derive(Default, SpreadAllocate, PSP34Storage, PSP34MetadataStorage)]
    #[ink(storage)]
    pub struct MyPSP34 {
        #[PSP34StorageField]
        psp34: PSP34Data,
        #[PSP34MetadataStorageField]
        metadata: PSP34MetadataData,
    }

    impl PSP34 for MyPSP34 {}

    impl PSP34Metadata for MyPSP34 {}

    impl MyPSP34 {
        /// A constructor which mints the first token to the owner
        #[ink(constructor)]
        pub fn new(id: Id, name: String, symbol: String) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                let name_key: Vec<u8> = String::from("name").into_bytes();
                let symbol_key: Vec<u8> = String::from("symbol").into_bytes();
                instance._set_attribute(id.clone(), name_key, name.into_bytes());
                instance._set_attribute(id, symbol_key, symbol.into_bytes());
            })
        }
    }
}
