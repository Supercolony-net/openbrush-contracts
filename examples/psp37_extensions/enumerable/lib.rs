#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp37_enumerable {
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::psp37::extensions::{
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
        psp37: psp37::Data<enumerable::Balances>,
    }

    impl PSP37 for Contract {}

    impl PSP37Mintable for Contract {}

    impl PSP37Burnable for Contract {}

    impl PSP37Enumerable for Contract {}

    impl PSP37Batch for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self| {})
        }
    }
}
