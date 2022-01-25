#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod my_psp721_mintable {
    use brush::contracts::psp721::extensions::mintable::*;
    use ink_prelude::string::String;

    #[derive(Default, PSP721Storage)]
    #[ink(storage)]
    pub struct MyPSP721 {
        #[PSP721StorageField]
        psp721: PSP721Data,
        // fields for hater logic
        hated_account: AccountId,
    }

    impl PSP721Internal for MyPSP721 {
        // Let's override method to reject transactions to bad account
        fn _before_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
            to: Option<&AccountId>,
            _id: &Id,
        ) -> Result<(), PSP721Error> {
            if to.unwrap() == &self.hated_account {
                return Err(PSP721Error::Custom(String::from("I hate this account!")))
            }
            Ok(())
        }
    }

    impl PSP721 for MyPSP721 {}

    impl PSP721Mintable for MyPSP721 {}

    impl MyPSP721 {
        /// The constructor
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
