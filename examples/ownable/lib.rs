#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod ownable {
    use ink::prelude::vec::Vec;
    use openbrush::{
        contracts::{
            ownable::*,
            psp37::extensions::{
                burnable::*,
                mintable::*,
            },
        },
        modifiers,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp37: psp37::Data,
        #[storage_field]
        ownable: ownable::Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            instance._init_with_owner(Self::env().caller());
            instance
        }
    }

    impl Ownable for Contract {}

    impl PSP37 for Contract {}

    impl PSP37Mintable for Contract {
        #[ink(message)]
        #[modifiers(only_owner)]
        fn mint(&mut self, to: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP37Error> {
            self._mint_to(to, ids_amounts)
        }
    }

    impl PSP37Burnable for Contract {
        #[ink(message)]
        #[modifiers(only_owner)]
        fn burn(&mut self, from: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP37Error> {
            self._burn_from(from, ids_amounts)
        }
    }
}
