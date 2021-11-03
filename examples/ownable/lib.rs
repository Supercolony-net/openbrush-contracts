#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod ownable {
    use brush::{
        modifiers,
        traits::InkStorage,
    };
    use ink_prelude::vec::Vec;
    use ownable::traits::*;
    use psp1155::{
        extensions::{
            burnable::*,
            mintable::*,
        },
        traits::*,
    };

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
        fn mint_to(&mut self, to: AccountId, id: Id, amount: Balance) -> Result<(), PSP1155Error> {
            self._mint(to, id, amount)
        }

        #[ink(message)]
        #[modifiers(only_owner)]
        fn mint(&mut self, id: Id, amount: Balance) -> Result<(), PSP1155Error> {
            self._mint(Self::env().caller(), id, amount)
        }
    }

    impl PSP1155Burnable for PSP1155Struct {
        #[ink(message)]
        #[modifiers(only_owner)]
        fn burn(&mut self, id: Id, amount: Balance) {
            self._burn(Self::env().caller(), id, amount)
        }
    }
}
