#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract]
pub mod erc721_receiver {
    use ink::prelude::vec::Vec;
    use openbrush::{
        contracts::traits::psp34::*,
        traits::String,
    };

    #[ink(storage)]
    pub struct PSP34ReceiverStruct {
        call_counter: u64,
    }

    impl PSP34ReceiverStruct {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { call_counter: 0 }
        }

        #[ink(message)]
        pub fn get_call_counter(&self) -> u64 {
            self.call_counter
        }
    }
}
