#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp34_mintable {
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::psp34::extensions::mintable::*,
        traits::Storage,
    };

    #[derive(Default, SpreadAllocate, Storage)]
    #[ink(storage)]
    pub struct MyPSP34 {
        #[storage_field]
        psp34: psp34::Data,
    }

    impl PSP34 for MyPSP34 {}

    impl PSP34Mintable for MyPSP34 {}

    impl MyPSP34 {
        /// The constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self| {})
        }
    }
}
