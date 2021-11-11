#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod psp3156_flash_borrower {
    use ink_prelude::vec::Vec;
    use psp22::extensions::flashmint::*;

    #[ink(storage)]
    pub struct PSP3156FlashBorrowerStruct {}

    impl PSP3156FlashBorrowerStruct {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
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
            brush::blake2b_256!("PSP3156FlashBorrower.onFlashLoan")
        }
    }
}
