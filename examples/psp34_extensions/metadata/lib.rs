#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp34_metadata {
    use ink_prelude::vec::Vec;
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::psp34::extensions::metadata::*,
        traits::{
            Storage,
            String,
        },
    };

    #[derive(Default, SpreadAllocate, Storage)]
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
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                let name_key: Vec<u8> = String::from("name").into_bytes();
                let symbol_key: Vec<u8> = String::from("symbol").into_bytes();
                instance._set_attribute(id.clone(), name_key, name.into_bytes());
                instance._set_attribute(id, symbol_key, symbol.into_bytes());
            })
        }
    }
}
