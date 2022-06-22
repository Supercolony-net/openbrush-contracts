#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp35 {
    use ink_prelude::{
        string::String,
        vec,
    };
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::psp35::*,
        storage::Mapping,
    };

    #[derive(Default, SpreadAllocate, PSP35Storage)]
    #[ink(storage)]
    pub struct MyPSP35 {
        #[PSP35StorageField]
        psp35: PSP35Data,
        denied_ids: Mapping<Id, ()>,
    }

    impl PSP35 for MyPSP35 {}

    impl MyPSP35 {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self| {})
        }

        #[ink(message)]
        pub fn deny(&mut self, id: Id) {
            self.denied_ids.insert(&id, &());
        }

        #[ink(message)]
        pub fn mint_tokens(&mut self, id: Id, amount: Balance) -> Result<(), PSP35Error> {
            if self.denied_ids.get(&id).is_some() {
                return Err(PSP35Error::Custom(String::from("Id is denied")))
            }
            self._mint_to(Self::env().caller(), vec![(id, amount)])
        }
    }
}
