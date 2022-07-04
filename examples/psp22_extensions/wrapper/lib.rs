#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22_wrapper {
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::psp22::extensions::wrapper::*,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct MyPSP22Wrapper {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        wrapper: wrapper::Data,
    }

    impl PSP22 for MyPSP22Wrapper {}

    impl PSP22Wrapper for MyPSP22Wrapper {}

    impl MyPSP22Wrapper {
        #[ink(constructor)]
        pub fn new(token_address: AccountId) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
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
