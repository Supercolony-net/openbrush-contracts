#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod my_psp22_mintable {
    use ink_prelude::vec::Vec;
    use psp22::{
        extensions::mintable::*,
        traits::*,
    };

    #[ink(storage)]
    #[derive(Default, PSP22Storage)]
    pub struct MyPSP22 {
        #[PSP22StorageField]
        psp22: PSP22Data,
    }

    impl PSP22 for MyPSP22 {}

    impl PSP22Mintable for MyPSP22 {}

    impl MyPSP22 {
        #[ink(constructor)]
        pub fn new(_total_supply: Balance) -> Self {
            let mut instance = Self::default();
            instance._mint(instance.env().caller(), _total_supply);
            instance
        }

        #[ink(message)]
        pub fn mint_to(&mut self, account: AccountId, amount: Balance) {
            self.mint(account, amount);
        }
    }
}
