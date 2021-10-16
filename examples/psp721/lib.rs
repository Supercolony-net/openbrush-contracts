#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod my_psp721 {
    use psp721::{
        extensions::{
            burnable::*,
            mintable::*,
        },
        traits::*,
    };

    #[derive(Default, PSP721Storage, PSP721MetadataStorage)]
    #[ink(storage)]
    pub struct MyPSP721 {
        #[PSP721StorageField]
        psp721: PSP721Data,
        #[PSP721MetadataStorageField]
        metadata: PSP721MetadataData,
        next_id: u8,
    }

    impl PSP721 for MyPSP721 {}

    impl PSP721Burnable for MyPSP721 {}

    impl PSP721Mintable for MyPSP721 {}

    impl MyPSP721 {
        /// A constructor which mints the first token to the owner
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            instance.mint_token();
            instance
        }

        /// Mint method which mints a token and updates the id of next token
        fn mint_token(&mut self) {
            self.mint([self.next_id; 32]);
            self.next_id += 1;
        }
    }
}
