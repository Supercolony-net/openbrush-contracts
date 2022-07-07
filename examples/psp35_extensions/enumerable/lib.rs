#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp35_enumerable {
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::psp35::extensions::{
            batch::*,
            burnable::*,
            enumerable::*,
            mintable::*,
        },
        traits::Storage,
    };

    #[derive(Default, SpreadAllocate, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        psp35: psp35::Data<enumerable::Balances>,
    }

    impl PSP35 for Contract {}

    impl PSP35Mintable for Contract {}

    impl PSP35Burnable for Contract {}

    impl PSP35Enumerable for Contract {}

    impl PSP35Batch for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self| {})
        }
    }
}
