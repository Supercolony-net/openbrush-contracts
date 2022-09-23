#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp37 {
    use ink_prelude::vec;
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::psp37::*,
        storage::Mapping,
        traits::{
            Storage,
            String,
        },
    };

    #[derive(Default, SpreadAllocate, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        psp37: psp37::Data,
        denied_ids: Mapping<Id, ()>,
    }

    impl PSP37 for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self| {})
        }

        #[ink(message)]
        pub fn deny(&mut self, id: Id) {
            self.denied_ids.insert(&id, &());
        }

        #[ink(message)]
        pub fn mint_tokens(&mut self, id: Id, amount: Balance) -> Result<(), PSP37Error> {
            if self.denied_ids.get(&id).is_some() {
                return Err(PSP37Error::Custom(String::from("Id is denied")))
            }
            self._mint_to(Self::env().caller(), vec![(id, amount)])
        }
    }
}
