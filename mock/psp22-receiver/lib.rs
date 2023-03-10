#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract]
pub mod erc20_receiver {
    use ink::prelude::vec::Vec;
    use openbrush::{
        contracts::traits::psp22::*,
        traits::String,
    };

    #[ink(storage)]
    pub struct PSP22ReceiverStruct {
        call_counter: u64,
        revert_next_transfer: bool,
    }

    impl PSP22ReceiverStruct {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                call_counter: 0,
                revert_next_transfer: false,
            }
        }

        #[ink(message)]
        pub fn get_call_counter(&self) -> u64 {
            self.call_counter
        }

        #[ink(message)]
        pub fn revert_next_transfer(&mut self) {
            self.revert_next_transfer = true
        }
    }
}
