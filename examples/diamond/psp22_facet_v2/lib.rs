#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22_facet_v2 {
    use ink_lang::codegen::Env;
    use ink_prelude::vec::Vec;
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::{
            ownable::*,
            psp22::*,
        },
        traits::ZERO_ADDRESS,
    };

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, PSP22Storage, OwnableStorage)]
    pub struct PSP22FacetV2 {
        #[PSP22StorageField]
        psp22: PSP22Data,
        #[OwnableStorageField]
        ownable: OwnableData,
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

    impl Ownable for PSP22FacetV2 {}

    impl PSP22FacetV2 {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut PSP22FacetV2| {
                instance._init_with_owner(instance.env().caller());
            })
        }
    }
}
