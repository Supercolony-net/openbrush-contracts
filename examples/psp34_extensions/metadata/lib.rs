#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod my_psp34_metadata {
    use brush::contracts::psp34::extensions::metadata::*;
    use ink_prelude::string::String;

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

    impl MyPSP34 {
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
