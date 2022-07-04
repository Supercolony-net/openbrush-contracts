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
    pub struct MyPSP35 {
        #[storage_field]
        psp35: psp35::Data<enumerable::Balances>,
    }

    impl PSP35 for MyPSP35 {}

    impl PSP35Mintable for MyPSP35 {}

    impl PSP35Burnable for MyPSP35 {}

    impl PSP35Enumerable for MyPSP35 {}

    impl PSP35Batch for MyPSP35 {}

    impl MyPSP35 {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self| {})
        }
    }
}
