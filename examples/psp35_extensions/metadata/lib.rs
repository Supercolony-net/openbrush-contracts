#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp35 {
    use ink_prelude::vec::Vec;
    use ink_storage::traits::SpreadAllocate;
    use openbrush::contracts::psp35::extensions::metadata::*;

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
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self| {})
        }

        #[ink(message)]
        pub fn set_attribute(&mut self, id: Id, key: Vec<u8>, data: Vec<u8>) -> Result<(), PSP35Error> {
            self._set_attribute(&id, &key, &data)
        }
    }
}
