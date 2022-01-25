#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod my_psp1155 {
    use brush::contracts::psp1155::*;
    use ink_prelude::{
        string::String,
        vec,
        vec::Vec
    };
    use ink_storage::collections::HashMap as StorageHashMap;

    #[derive(Default, PSP1155Storage)]
    #[ink(storage)]
    pub struct MyPSP1155 {
        #[PSP1155StorageField]
        psp1155: PSP1155Data,
        denied_ids: StorageHashMap<Id, ()>,
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

    impl MyPSP1155 {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn deny(&mut self, id: Id) {
            self.denied_ids.insert(id, ());
        }

        #[ink(message)]
        pub fn mint_tokens(&mut self, id: Id, amount: Balance) -> Result<(), PSP1155Error> {
            if self.denied_ids.get(&id).is_some() {
                return Err(PSP1155Error::Custom(String::from("Id is denied")))
            }
            self._mint_to(Self::env().caller(), vec![(id, amount)])
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
