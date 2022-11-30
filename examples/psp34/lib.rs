#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp34 {
    use openbrush::{
        contracts::psp34::extensions::metadata::*,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        metadata: Data,
        next_id: u8,
    }

    impl PSP34 for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn mint_token(&mut self) -> Result<(), PSP34Error> {
            self._mint_to(Self::env().caller(), Id::U8(self.next_id))?;
            self.next_id += 1;
            Ok(())
        }

        #[ink(message)]
        pub fn mint(&mut self, id: Id) -> Result<(), PSP34Error> {
            self._mint_to(Self::env().caller(), id)
        }
    }
}
