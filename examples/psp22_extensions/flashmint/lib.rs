#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22_flashmint {
    use openbrush::{
        contracts::psp22::extensions::flashmint::*,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp22: psp22::Data,
    }

    impl PSP22 for Contract {}

    impl FlashLender for Contract {}

    // To override an internal method from OpenBrush implementation
    // you need to override that in the `PSP22FlashLenderInternal` trait
    impl Internal for Contract {
        /// Override `get_fee` function to add 1% fee to the borrowed `amount`
        fn _get_fee(&self, amount: Balance) -> Balance {
            amount / 100
        }
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self::default();
            assert!(instance._mint_to(Self::env().caller(), total_supply).is_ok());

            instance
        }
    }
}
