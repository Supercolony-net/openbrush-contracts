#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp35 {
    use ink_storage::traits::SpreadAllocate;
    use openbrush::contracts::psp35::extensions::mintable::*;

    #[derive(Default, SpreadAllocate, PSP35Storage)]
    #[ink(storage)]
    pub struct MyPSP35 {
        #[PSP35StorageField]
        psp35: PSP35Data,
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
