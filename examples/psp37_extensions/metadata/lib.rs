#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp37 {
    use ink::prelude::vec::Vec;
    use ink::storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::psp37::extensions::metadata::*,
        traits::Storage,
    };

    #[derive(Default, SpreadAllocate, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        psp37: psp37::Data,
        #[storage_field]
        metadata: metadata::Data,
    }

    impl PSP37 for Contract {}

    impl PSP37Metadata for Contract {}

    impl Contract {
        /// contract constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            ink::codegen::initialize_contract(|_instance: &mut Self| {})
        }

        #[ink(message)]
        pub fn set_attribute(&mut self, id: Id, key: Vec<u8>, data: Vec<u8>) -> Result<(), PSP37Error> {
            self._set_attribute(&id, &key, &data)
        }
    }
}
