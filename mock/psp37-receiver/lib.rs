#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract]
pub mod psp37_receiver {
    use ink::prelude::vec::Vec;
    use openbrush::{
        contracts::traits::psp37::*,
        traits::String,
    };

    #[ink(storage)]
    pub struct PSP37ReceiverStruct {
        call_counter: u64,
        revert_next_transfer: bool,
    }

    impl PSP37ReceiverStruct {
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
