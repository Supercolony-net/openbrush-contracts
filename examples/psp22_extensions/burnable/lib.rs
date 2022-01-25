#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod my_psp22_burnable {
    use brush::contracts::psp22::extensions::burnable::*;
    use ink_prelude::{
        vec::Vec,
        string::String
    };

    #[ink(storage)]
    #[derive(Default, PSP22Storage)]
    pub struct MyPSP22 {
        #[PSP22StorageField]
        psp22: PSP22Data,
        // fields for hater logic
        hated_account: AccountId,
    }

    impl PSP22Internal for MyPSP22 {
        // Let's override method to reject transactions to bad account
        fn _before_token_transfer(
            &mut self,
            from: Option<&AccountId>,
            _to: Option<&AccountId>,
            _amount: &Balance,
        ) -> Result<(), PSP22Error> {
            if from.is_some() && from.unwrap() == &self.hated_account {
                return Err(PSP22Error::Custom(String::from("I hate this account!")))
            }
            Ok(())
        }
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
                self.burn(account.0, account.1)?;
            }
            Ok(())
        }

        #[ink(message)]
        pub fn set_hated_account(&mut self, hated: AccountId) {
            self.hated_account = hated;
        }

        #[ink(message)]
        pub fn get_hated_account(&self) -> AccountId {
            self.hated_account.clone()
        }
    }
}
