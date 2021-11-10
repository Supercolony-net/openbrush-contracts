#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod my_psp721_metadata {
    use ink_prelude::string::String;
    use psp721::{
        extensions::metadata::*,
        traits::*,
    };

    #[derive(Default, PSP721Storage, PSP721MetadataStorage)]
    #[ink(storage)]
    pub struct MyPSP721 {
        #[PSP721StorageField]
        psp721: PSP721Data,
        #[PSP721MetadataStorageField]
        metadata: PSP721MetadataData,
    }

    impl PSP721 for MyPSP721 {}

    impl PSP721Metadata for MyPSP721 {}

    impl MyPSP721 {
        /// A constructor which mints the first token to the owner
        #[ink(constructor)]
        pub fn new(name: Option<String>, symbol: Option<String>) -> Self {
            let mut instance = Self::default();
            instance.metadata.name = name;
            instance.metadata.symbol = symbol;
            instance
        }
    }
}
