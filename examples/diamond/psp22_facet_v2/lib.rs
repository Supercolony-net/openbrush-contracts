#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22_facet_v2 {
    use ink::{
        codegen::Env,
        prelude::vec::Vec,
    };
    use openbrush::{
        contracts::{
            ownable::*,
            psp22::*,
        },
        traits::{
            Storage,
            ZERO_ADDRESS,
        },
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct PSP22FacetV2 {
        #[storage_field]
        psp22: psp22::Data,
        // Ownable is used only internally without exposing it to the world
        #[storage_field]
        ownable: ownable::Data,
    }

    impl PSP22 for PSP22FacetV2 {
        #[ink(message)]
        fn transfer(&mut self, to: AccountId, value: Balance, data: Vec<u8>) -> Result<(), PSP22Error> {
            let from = self.env().caller();
            let is_tax = to != ZERO_ADDRESS.into() && from != ZERO_ADDRESS.into();
            // we will burn 10% of transfer to and from non-zero accounts
            let burned = if is_tax { value / 10 } else { 0 };
            if is_tax {
                self._burn_from(from, burned)?;
            }
            self._transfer_from_to(from, to, value - burned, data)?;
            Ok(())
        }
    }

    impl PSP22FacetV2 {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink::codegen::initialize_contract(|_instance: &mut Self| {})
        }
    }
}
