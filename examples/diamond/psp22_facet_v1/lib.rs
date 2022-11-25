#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22_facet_v1 {
    use ink::storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::{
            ownable::*,
            psp22::*,
        },
        modifiers,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct PSP22FacetV1 {
        #[storage_field]
        psp22: psp22::Data,
        // Ownable is used only internally without exposing it to the world
        #[storage_field]
        ownable: ownable::Data,
    }

    impl PSP22 for PSP22FacetV1 {}

    impl PSP22FacetV1 {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink::codegen::initialize_contract(|_instance: &mut Self| {})
        }

        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn init_psp22(&mut self) -> Result<(), PSP22Error> {
            self._mint_to(Self::env().caller(), 1000)
        }
    }
}
