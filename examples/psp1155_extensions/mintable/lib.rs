#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod my_psp1155 {
    use brush::contracts::psp1155::extensions::mintable::*;
    use ink_prelude::{
        string::String,
        vec::Vec
    };

    #[derive(Default, PSP1155Storage)]
    #[ink(storage)]
    pub struct MyPSP1155 {
        #[PSP1155StorageField]
        psp1155: PSP1155Data,
        // fields for hater logic
        hated_account: AccountId,
    }

    impl PSP1155Internal for MyPSP1155 {
        // Let's override method to reject transactions to bad account
        fn _before_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
            to: Option<&AccountId>,
            _ids: &Vec<(Id, Balance)>,
        ) -> Result<(), PSP1155Error> {
            if to.unwrap() == &self.hated_account {
                return Err(PSP1155Error::Custom(String::from("I hate this account!")))
            }
            Ok(())
        }
    }

    impl PSP1155 for MyPSP1155 {}

    impl PSP1155Mintable for MyPSP1155 {}

    impl MyPSP1155 {
        /// contract constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
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
