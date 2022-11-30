#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp34_enumerable {
    use openbrush::{
        contracts::psp34::extensions::{
            burnable::*,
            enumerable::*,
            mintable::*,
        },
        traits::Storage,
    };

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        psp34: psp34::Data<enumerable::Balances>,
    }

    impl PSP34 for Contract {}

    impl PSP34Mintable for Contract {}

    impl PSP34Burnable for Contract {}

    impl PSP34Enumerable for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
    }
}
