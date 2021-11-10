#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod my_psp721_mintable {
    use psp721::{
        extensions::mintable::*,
        traits::*,
    };

    #[derive(Default, PSP721Storage)]
    #[ink(storage)]
    pub struct MyPSP721 {
        #[PSP721StorageField]
        psp721: PSP721Data,
    }

    impl PSP721 for MyPSP721 {}

    impl PSP721Mintable for MyPSP721 {}

    impl MyPSP721 {
        /// The constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
    }
}
