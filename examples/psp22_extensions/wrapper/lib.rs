#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod my_psp22_wrapper {
    use psp22::{
        extensions::wrapper::*,
        traits::*,
    };

    #[ink(storage)]
    #[derive(Default, PSP22WrapperStorage, PSP22Storage)]
    pub struct MyPSP22Wrapper {
        #[PSP22StorageField]
        psp22: PSP22Data,
        #[PSP22WrapperStorageField]
        wrapper: PSP22WrapperData,
    }

    impl PSP22 for MyPSP22Wrapper {}

    impl PSP22Wrapper for MyPSP22Wrapper {}

    impl MyPSP22Wrapper {
        #[ink(constructor)]
        pub fn new(token_address: AccountId) -> Self {
            let mut instance = Self::default();
            instance._init(token_address);
            instance
        }

        /// Exposes the `_recover` function for message caller
        #[ink(message)]
        pub fn recover(&mut self) -> Result<Balance, PSP22Error> {
            self._recover(Self::env().caller())
        }
    }
}
