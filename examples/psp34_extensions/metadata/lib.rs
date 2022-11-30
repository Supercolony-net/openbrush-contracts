#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp34_metadata {
    use ink::prelude::vec::Vec;
    use openbrush::{
        contracts::psp34::extensions::metadata::*,
        traits::{
            Storage,
            String,
        },
    };

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        metadata: metadata::Data,
    }

    impl PSP34 for Contract {}

    impl PSP34Metadata for Contract {}

    impl Contract {
        /// A constructor which mints the first token to the owner
        #[ink(constructor)]
        pub fn new(id: Id, name: String, symbol: String) -> Self {
            let mut instance = Self::default();

            let name_key: Vec<u8> = String::from("name");
            let symbol_key: Vec<u8> = String::from("symbol");
            instance._set_attribute(id.clone(), name_key, name);
            instance._set_attribute(id, symbol_key, symbol);

            instance
        }
    }
}
