#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp34_burnable {
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::psp34::extensions::burnable::*,
        traits::Storage,
    };

    #[derive(Default, SpreadAllocate, Storage)]
    #[ink(storage)]
    pub struct MyPSP34 {
        #[storage_field]
        psp34: psp34::Data,
    }

    impl PSP34 for MyPSP34 {}

    impl PSP34Burnable for MyPSP34 {}

    impl MyPSP34 {
        /// The constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance
                    ._mint_to(Self::env().caller(), Id::U8(0u8))
                    .expect("Should mint token with id 0");
                instance
                    ._mint_to(Self::env().caller(), Id::U8(1u8))
                    .expect("Should mint token with id 1");
                instance
                    ._mint_to(Self::env().caller(), Id::U8(2u8))
                    .expect("Should mint token with id 2");
            })
        }
    }
}
