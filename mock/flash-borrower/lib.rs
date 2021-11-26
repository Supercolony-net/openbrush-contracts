#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod flash_borrower {
    use ink_prelude::vec::Vec;
    use psp22::{
        extensions::flashmint::*,
        traits::PSP22Caller,
    };

    #[ink(storage)]
    pub struct FlashBorrowerStruct {}

    impl FlashBorrowerStruct {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }
    }

    impl FlashBorrower for FlashBorrowerStruct {
        #[ink(message)]
        fn on_flashloan(
            &mut self,
            initiator: AccountId,
            token: AccountId,
            amount: Balance,
            fee: Balance,
            _data: Vec<u8>,
        ) -> Result<(), FlashBorrowerError> {
            PSP22Caller::approve(&token, initiator, amount + fee)?;
            // do something with the tokens
            Ok(())
        }
    }
}
