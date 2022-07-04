#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp34_enumerable {
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::psp34::extensions::{
            burnable::*,
            enumerable::*,
            mintable::*,
        },
        traits::Storage,
    };

    #[derive(Default, SpreadAllocate, Storage)]
    #[ink(storage)]
    pub struct MyPSP34 {
        #[storage_field]
        psp34: psp34::Data<enumerable::Balances>,
    }

    impl PSP34 for MyPSP34 {}

    impl PSP34Mintable for MyPSP34 {}

    impl PSP34Burnable for MyPSP34 {}

    impl PSP34Enumerable for MyPSP34 {}

    impl MyPSP34 {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self| {})
        }
    }
}
