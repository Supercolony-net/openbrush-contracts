#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod flash_borrower {
    use ink_prelude::vec::Vec;
    use psp22::{
        extensions::flashmint::*,
        traits::{
            PSP22Caller,
            PSP22Error,
        },
    };

    #[ink(storage)]
    pub struct FlashBorrowerStruct {}

    impl FlashBorrowerStruct {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn approve_token(
            &mut self,
            token_address: AccountId,
            spender: AccountId,
            amount: Balance,
        ) -> Result<(), PSP22Error> {
            PSP22Caller::approve(&token_address, spender, amount)
        }
    }

    impl FlashBorrower for FlashBorrowerStruct {
        #[ink(message)]
        fn on_flashloan(
            &mut self,
            _initiator: AccountId,
            _token: AccountId,
            _amount: Balance,
            _fee: Balance,
            _data: Vec<u8>,
        ) -> Result<(), FlashBorrowerError> {
            // do something with the tokens
            Ok(())
        }
    }
}
