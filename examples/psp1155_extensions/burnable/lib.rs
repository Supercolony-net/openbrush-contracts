#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod my_psp1155 {
    use brush::contracts::psp1155::extensions::burnable::*;
    use ink_prelude::{
        string::String,
        vec,
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
            from: Option<&AccountId>,
            _to: Option<&AccountId>,
            _ids: &Vec<(Id, Balance)>,
        ) -> Result<(), PSP1155Error> {
            if from.is_some() && from.unwrap() == &self.hated_account {
                return Err(PSP1155Error::Custom(String::from("I hate this account!")))
            }
            Ok(())
        }
    }

    impl PSP1155 for MyPSP1155 {}

    impl PSP1155Burnable for MyPSP1155 {}

    impl MyPSP1155 {
        /// contract constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            let caller = Self::env().caller();
            instance
                ._mint_to(caller, vec![([0; 32], 1)])
                .expect("Should mint token");
            let mut id = [0; 32];
            id[0] = 1;
            instance._mint_to(caller, vec![(id, 20)]).expect("Should mint token");
            instance
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
