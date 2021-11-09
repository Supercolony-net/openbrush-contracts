#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod my_psp22_burnable {
    use ink_prelude::vec::Vec;
    use psp22::{
        extensions::burnable::*,
        traits::*,
    };

    #[ink(storage)]
    #[derive(Default, PSP22Storage)]
    pub struct MyPSP22 {
        #[PSP22StorageField]
        psp22: PSP22Data,
    }

    impl PSP22 for MyPSP22 {}

    impl PSP22Burnable for MyPSP22 {}

    impl MyPSP22 {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self::default();
            assert!(instance._mint(instance.env().caller(), total_supply).is_ok());
            instance
        }

        #[ink(message)]
        pub fn burn_from_many(&mut self, accounts: Vec<(AccountId, Balance)>) -> Result<(), PSP22Error> {
            for account in accounts.iter() {
                self.burn_from(account.0, account.1)?;
            }
            Ok(())
        }
    }
}
