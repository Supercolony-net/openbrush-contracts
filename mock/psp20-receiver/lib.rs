#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod erc20_receiver {
    use psp20::traits::*;

    #[ink(storage)]
    pub struct PSP20ReceiverStruct {
        call_counter: u64,
        revert_next_transfer: bool
    }

    impl PSP20ReceiverStruct {
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

    impl PSP20Receiver for PSP20ReceiverStruct {
        fn on_received(
                &mut self,
                _operator: AccountId,
                _from: AccountId,
                _value: Balance,
                _data: Vec<u8>) -> Result<(), PSP20ReceiverError> {
            if self.revert_next_transfer {
                self.revert_next_transfer = false;
                return Err(PSP20ReceiverError::TransferRejected(String::from("Transfer Rejected")))
            }
            self.call_counter += 1;
            Ok(())
        }
    }
}