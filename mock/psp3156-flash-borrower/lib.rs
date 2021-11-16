#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod psp3156_flash_borrower {
    use ink_prelude::vec::Vec;
    use psp22::{
        extensions::flashmint::*,
        traits::{
            PSP22Error,
            PSP22Wrapper,
        },
    };

    #[ink(storage)]
    pub struct PSP3156FlashBorrowerStruct {}

    impl PSP3156FlashBorrowerStruct {
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
            PSP22Wrapper::approve(&token_address, spender, amount)
        }
    }

    impl PSP3156FlashBorrower for PSP3156FlashBorrowerStruct {
        #[ink(message)]
        fn on_flashloan(
            &mut self,
            _initiator: AccountId,
            _token: AccountId,
            _amount: Balance,
            _fee: Balance,
            _data: Vec<u8>,
        ) -> [u8; 32] {
            // do something with the tokens
            ink_lang::blake2x256!("PSP3156FlashBorrower.onFlashLoan")
        }
    }
}
