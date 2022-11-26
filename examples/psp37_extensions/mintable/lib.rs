#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp37 {
    use openbrush::{
        contracts::psp37::extensions::mintable::*,
        traits::Storage,
    };

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        psp37: psp37::Data,
    }

    impl PSP37 for Contract {}

    impl PSP37Mintable for Contract {}

    impl Contract {
        /// contract constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            ink::codegen::initialize_contract(|_instance: &mut Self| {})
        }
    }
}
