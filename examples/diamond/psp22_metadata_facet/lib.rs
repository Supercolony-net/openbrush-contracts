#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22_metadata_facet {
    use ink_prelude::string::String;
    use openbrush::{
        contracts::{
            ownable::*,
            psp22::extensions::metadata::*,
        },
        modifiers,
    };

    use ink_storage::traits::SpreadAllocate;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, PSP22MetadataStorage, OwnableStorage)]
    pub struct PSP22Facet {
        #[PSP22MetadataStorageField]
        metadata: PSP22MetadataData,
        #[OwnableStorageField]
        ownable: OwnableData,
    }

    impl PSP22Metadata for PSP22Facet {}

    impl Ownable for PSP22Facet {}

    impl PSP22Facet {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut PSP22Facet| {
                instance._init_with_owner(instance.env().caller());
                instance.init_metadata().expect("Can not set metadata");
            })
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
