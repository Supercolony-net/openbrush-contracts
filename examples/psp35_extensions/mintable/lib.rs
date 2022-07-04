#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp35 {
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::psp35::extensions::mintable::*,
        traits::Storage,
    };

    #[derive(Default, SpreadAllocate, Storage)]
    #[ink(storage)]
    pub struct MyPSP35 {
        #[storage_field]
        psp35: psp35::Data,
    }

    impl PSP35 for MyPSP35 {}

    impl PSP35Mintable for MyPSP35 {}

    impl MyPSP35 {
        /// contract constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self| {})
        }
    }
}
