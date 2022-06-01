#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp1155 {
    use ink_prelude::string::String;
    use ink_storage::traits::SpreadAllocate;
    use openbrush::contracts::psp1155::extensions::metadata::*;

    #[derive(Default, SpreadAllocate, PSP1155Storage, PSP1155MetadataStorage)]
    #[ink(storage)]
    pub struct MyPSP1155 {
        #[PSP1155StorageField]
        psp1155: PSP1155Data,
        #[PSP1155MetadataStorageField]
        metadata: PSP1155MetadataData,
    }

    impl PSP1155 for MyPSP1155 {}

    impl PSP1155Metadata for MyPSP1155 {}

    impl MyPSP1155 {
        /// contract constructor
        #[ink(constructor)]
        pub fn new(uri: Option<String>) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.metadata.uri = uri;
            })
        }
    }
}
