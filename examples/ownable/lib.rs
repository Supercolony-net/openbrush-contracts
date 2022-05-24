#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod ownable {
    use ink_prelude::vec::Vec;
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::{
            ownable::*,
            psp35::extensions::{
                burnable::*,
                mintable::*,
            },
        },
        modifiers,
    };

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, PSP35Storage, OwnableStorage)]
    pub struct PSP35Struct {
        #[PSP35StorageField]
        psp1155: PSP35Data,
        #[OwnableStorageField]
        ownable: OwnableData,
    }

    impl PSP35Struct {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                let caller = instance.env().caller();
                instance._init_with_owner(caller);
            })
        }
    }

    impl Ownable for PSP35Struct {}

    impl PSP35 for PSP35Struct {}

    impl PSP35Mintable for PSP35Struct {
        #[ink(message)]
        #[modifiers(only_owner)]
        fn mint(&mut self, to: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP35Error> {
            self._mint_to(to, ids_amounts)
        }
    }

    impl PSP35Burnable for PSP35Struct {
        #[ink(message)]
        #[modifiers(only_owner)]
        fn burn(&mut self, from: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP35Error> {
            self._burn_from(from, ids_amounts)
        }
    }
}
