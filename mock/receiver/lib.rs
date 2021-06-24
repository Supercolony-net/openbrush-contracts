#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod receiver {
    use psp1155::{
        traits::{Id, IPSP1155Receiver, PSP1155ReceiverError},
    };

    #[ink(storage)]
    pub struct PSP1155ReceiverStruct {
        call_counter: u64
    }

    impl PSP1155ReceiverStruct {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { call_counter: 0 }
        }

        #[ink(message)]
        pub fn get_call_counter(&self) -> u64 {
            self.call_counter
        }
    }

    impl IPSP1155Receiver for PSP1155ReceiverStruct {
        #[ink(message)]
        fn on_psp1155_received(&mut self, _operator: AccountId, _from: AccountId,
                                   _id: Id, _value: Balance, _data: Vec<u8>) -> Result<(), PSP1155ReceiverError> {
            self.call_counter += 1;
            Ok(())
        }

        #[ink(message)]
        fn on_psp1155_batch_received(&mut self, _operator: AccountId, _from: AccountId,
                                         _ids: Vec<Id>, _values: Vec<Balance>, _data: Vec<u8>) -> Result<(), PSP1155ReceiverError> {
            self.call_counter += 1;
            Ok(())
        }
    }
}