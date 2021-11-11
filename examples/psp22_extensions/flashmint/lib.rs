#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod my_psp22_flashmint {
    use psp22::{
        extensions::flashmint::*,
        traits::*,
    };

    #[ink(storage)]
    #[derive(Default, PSP22Storage)]
    pub struct MyPSP22FlashMint {
        #[PSP22StorageField]
        psp22: PSP22Data,
    }

    impl PSP22 for MyPSP22FlashMint {}

    impl PSP22FlashMint for MyPSP22FlashMint {
        // we will add 1% fee to the amount
        fn get_fee(&mut self, amount: Balance) -> Balance {
            amount / 100
        }
    }

    impl PSP3156FlashBorrower for MyPSP22FlashMint {
        #[ink(message)]
        fn on_flashloan(
            &mut self,
            _initiator: AccountId,
            _token: AccountId,
            _amount: Balance,
            _fee: Balance,
            _data: Vec<u8>,
        ) -> [u8; 32] {
            // TODO do something with the tokens
            brush::blake2b_256!("PSP3156FlashBorrower.onFlashLoan")
        }
    }

    impl MyPSP22FlashMint {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
    }
}
