#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp34_mintable {
    use openbrush::{
        contracts::psp34::extensions::mintable::*,
        traits::Storage,
    };

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        psp34: psp34::Data,
    }

    impl PSP34 for Contract {}

    impl PSP34Mintable for Contract {}

    impl Contract {
        /// The constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
    }
}
