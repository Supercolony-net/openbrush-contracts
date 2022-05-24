#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod my_psp35 {
    use brush::contracts::psp35::extensions::metadata::*;
    use ink_prelude::vec::Vec;
    use ink_storage::traits::SpreadAllocate;

    #[derive(Default, SpreadAllocate, PSP35Storage, PSP35MetadataStorage)]
    #[ink(storage)]
    pub struct MyPSP35 {
        #[PSP35StorageField]
        psp35: PSP35Data,
        #[PSP35MetadataStorageField]
        metadata: PSP35MetadataData,
    }

    impl PSP35 for MyPSP35 {}

    impl PSP35Metadata for MyPSP35 {}

    impl MyPSP35 {
        /// contract constructor
        #[ink(constructor)]
        pub fn new(id: Id, key: Vec<u8>, attribute: Vec<u8>) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.metadata.attributes.insert(&(id, key), &attribute);
            })
        }
    }
}
