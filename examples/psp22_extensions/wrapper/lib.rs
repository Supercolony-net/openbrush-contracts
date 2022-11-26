#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22_wrapper {
    use openbrush::{
        contracts::psp22::extensions::wrapper::*,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        wrapper: wrapper::Data,
    }

    impl PSP22 for Contract {}

    impl PSP22Wrapper for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new(token_address: AccountId) -> Self {
            ink::codegen::initialize_contract(|instance: &mut Self| {
                instance._init(token_address);
            })
        }

        /// Exposes the `_recover` function for message caller
        #[ink(message)]
        pub fn recover(&mut self) -> Result<Balance, PSP22Error> {
            self._recover(Self::env().caller())
        }
    }
}
