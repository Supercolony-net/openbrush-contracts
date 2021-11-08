#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod my_psp721_mintable {
    use ink_prelude::vec::Vec;
    use psp721::{
        extensions::mintable::*,
        traits::*,
    };

    #[derive(Default, PSP721Storage)]
    #[ink(storage)]
    pub struct MyPSP721 {
        #[PSP721StorageField]
        psp721: PSP721Data,
        next_id: u8,
    }

    impl PSP721 for MyPSP721 {}

    impl PSP721Mintable for MyPSP721 {}

    impl MyPSP721 {
        /// The constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        /// Mint method which mints a token and updates the id of next token
        #[ink(message)]
        pub fn mint_token(&mut self) -> Result<(), PSP721Error> {
            let result = self.mint([self.next_id; 32]);
            self.next_id += 1;
            result
        }
    }
}
