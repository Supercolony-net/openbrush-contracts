#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod erc721_receiver {
    use psp721::traits::*;
    use ink_prelude::{ string::String, vec::Vec };

    #[ink(storage)]
    pub struct PSP721ReceiverStruct {
        call_counter: u64,
        revert_next_transfer: bool
    }

    impl PSP721ReceiverStruct {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { call_counter: 0, revert_next_transfer: false }
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

    impl IPSP721Receiver for PSP721ReceiverStruct {
        #[ink(message)]
        fn on_psp721_received(
            &mut self,
            _operator: AccountId,
            _from: AccountId,
            _id: Id,
            _data: Vec<u8>,
        ) -> Result<(), PSP721ReceiverError> {
            if self.revert_next_transfer {
                self.revert_next_transfer = false;
                return Err(PSP721ReceiverError::TransferRejected(String::from("Transfer Rejected")))
            }
            self.call_counter += 1;
            Ok(())
        }
    }
}