#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22 {
    use openbrush::{
        contracts::psp22::*,
        traits::{
            Storage,
            String,
        },
    };

    #[ink(storage)]
    #[derive(Storage)]
    pub struct Contract {
        #[storage_field]
        psp22: psp22::Data,
        // fields for hater logic
        hated_account: AccountId,
    }

    impl Transfer for Contract {
        // Let's override method to reject transactions to bad account
        fn _before_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
            to: Option<&AccountId>,
            _amount: &Balance,
        ) -> Result<(), PSP22Error> {
            if to == Some(&self.hated_account) {
                return Err(PSP22Error::Custom(String::from("I hate this account!")))
            }
            Ok(())
        }
    }

    impl PSP22 for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self {
                psp22: Default::default(),
                hated_account: [0u8; 32].into(),
            };

            instance
                ._mint_to(Self::env().caller(), total_supply)
                .expect("Should mint");

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
