#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod ownable {
    use brush::{
        contracts::{
            ownable::*,
            psp1155::extensions::{
                burnable::*,
                mintable::*,
            },
        },
        modifiers,
        traits::InkStorage,
    };
    use ink_prelude::vec::Vec;

    #[ink(storage)]
    #[derive(Default, PSP1155Storage, OwnableStorage)]
    pub struct PSP1155Struct {
        #[PSP1155StorageField]
        psp1155: PSP1155Data,
        #[OwnableStorageField]
        ownable: OwnableData,
    }

    impl PSP1155Struct {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            let caller = instance.env().caller();
            instance._init_with_owner(caller);
            instance
        }
    }

    impl Ownable for PSP1155Struct {}

    impl PSP1155 for PSP1155Struct {}

    impl PSP1155Mintable for PSP1155Struct {
        #[ink(message)]
        #[modifiers(only_owner)]
        fn mint(&mut self, to: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP1155Error> {
            self._mint_to(to, ids_amounts)
        }
    }

    impl PSP1155Burnable for PSP1155Struct {
        #[ink(message)]
        #[modifiers(only_owner)]
        fn burn(&mut self, from: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP1155Error> {
            self._burn_from(from, ids_amounts)
        }
    }
}
