#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod my_psp22_wrapper {
    use ink_env::call::FromAccountId;
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
            let token: PSP22Stub = FromAccountId::from_account_id(token_address);
            instance.init(token);
            instance
        }
    }
}
