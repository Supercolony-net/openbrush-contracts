#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod my_psp22_mintable {
    use brush::contracts::psp22::extensions::mintable::*;
    use ink_prelude::string::String;

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
            _from: Option<&AccountId>,
            to: Option<&AccountId>,
            _amount: &Balance,
        ) -> Result<(), PSP22Error> {
            if to.unwrap() == &self.hated_account {
                return Err(PSP22Error::Custom(String::from("I hate this account!")))
            }
            Ok(())
        }
    }

    impl PSP22 for MyPSP22 {}
    impl PSP22Mintable for MyPSP22 {}

    impl MyPSP22 {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self::default();
            assert!(instance._mint(instance.env().caller(), total_supply).is_ok());
            instance
        }

        #[ink(message)]
        pub fn mint_to(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            self.mint(account, amount)
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
