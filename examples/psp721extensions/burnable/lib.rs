#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod my_psp721_burnable {
    use ink_prelude::vec::Vec;
    use psp721::{
        extensions::burnable::*,
        traits::*,
    };

    #[derive(Default, PSP721Storage)]
    #[ink(storage)]
    pub struct MyPSP721 {
        #[PSP721StorageField]
        psp721: PSP721Data,
    }

    impl PSP721 for MyPSP721 {}

    impl PSP721Burnable for MyPSP721 {}

    impl MyPSP721 {
        /// The constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            match instance._mint([0; 32]) {
                Ok(result) => result,
                Err(e) => panic!("{}", e.as_ref()),
            }
            match instance._mint([1; 32]) {
                Ok(result) => result,
                Err(e) => panic!("{}", e.as_ref()),
            }
            match instance._mint([2; 32]) {
                Ok(result) => result,
                Err(e) => panic!("{}", e.as_ref()),
            }
            instance
        }
    }
}
