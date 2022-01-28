#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod my_psp34_metadata {
    use brush::contracts::psp34::extensions::metadata::*;
    use ink_prelude::{
        string::String,
        vec::Vec,
    };

    #[derive(Default, PSP34Storage, PSP34MetadataStorage)]
    #[ink(storage)]
    pub struct MyPSP34 {
        #[PSP34StorageField]
        psp34: PSP34Data,
        #[PSP34MetadataStorageField]
        metadata: PSP34MetadataData,
    }

    impl PSP34 for MyPSP34 {}

    impl PSP34Metadata for MyPSP34 {}

    impl PSP34MetadataInternal for MyPSP34 {}

    impl MyPSP34 {
        /// A constructor which mints the first token to the owner
        #[ink(constructor)]
        pub fn new(id: Id, name: String, symbol: String) -> Self {
            let mut instance = Self::default();
            let mut name_key: Vec<u8> = String::from("name").into_bytes();
            let mut symbol_key: Vec<u8> = String::from("symbol").into_bytes();
            instance.metadata._set_attribute(id, name_key, name.into_bytes());
            instance.metadata._set_attribute(id, symbol_key, symbol.into_bytes());
            instance
        }
    }
}
