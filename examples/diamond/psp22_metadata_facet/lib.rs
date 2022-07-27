#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22_metadata_facet {
    use ink_prelude::string::String;
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::{
            ownable::*,
            psp22::extensions::metadata::*,
        },
        modifiers,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct PSP22Facet {
        #[storage_field]
        metadata: metadata::Data,
        // Ownable is used only internally without exposing it to the world
        #[storage_field]
        ownable: ownable::Data,
    }

    impl PSP22Metadata for PSP22Facet {}

    impl PSP22Facet {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self | {})
        }

        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn init_metadata(&mut self) -> Result<(), PSP22Error> {
            self.metadata.name = Some(String::from("PSP22 Diamond"));
            self.metadata.symbol = Some(String::from("PSP22D"));
            self.metadata.decimals = 18;
            Ok(())
        }
    }
}
