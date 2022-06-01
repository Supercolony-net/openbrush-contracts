#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22_facet_v1 {
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::{
            ownable::*,
            psp22::*,
        },
        modifiers,
    };

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, PSP22Storage, OwnableStorage)]
    pub struct PSP22FacetV1 {
        #[PSP22StorageField]
        psp22: PSP22Data,
        #[OwnableStorageField]
        ownable: OwnableData,
    }

    impl PSP22 for PSP22FacetV1 {}

    impl Ownable for PSP22FacetV1 {}

    impl PSP22FacetV1 {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut PSP22FacetV1| {
                instance._init_with_owner(instance.env().caller());
                instance.init_psp22().expect("Should initialize");
            })
        }

        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn init_psp22(&mut self) -> Result<(), PSP22Error> {
            self._mint(Self::env().caller(), 1000)
        }
    }
}
