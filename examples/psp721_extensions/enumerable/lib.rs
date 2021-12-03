#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod my_psp721_enumerable {
    use psp721::{
        extensions::enumerable::*,
        traits::*,
    };

    #[derive(Default, PSP721Storage, PSP721EnumerableStorage)]
    #[ink(storage)]
    pub struct MyPSP721 {
        #[PSP721StorageField]
        psp721: PSP721Data,
        #[PSP721EnumerableField]
        enumerable: PSP721EnumerableData,
    }

    impl PSP721 for MyPSP721 {}

    impl PSP721Enumerable for MyPSP721 {}

    impl MyPSP721 {
        /// The constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
    }
}
