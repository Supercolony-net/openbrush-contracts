#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod ownable {
    use psp1155::{
        traits::{IPSP1155, Id, IPSP1155MetadataURI, IPSP1155Mint},
        impls::{PSP1155Storage, PSP1155MetadataStorage, PSP1155Metadata, PSP1155, PSP1155Mint}
    };
    use ownable::{
        traits::{IOwnable, OwnableError},
        impls::{OwnableStorage, Ownable}
    };
    use brush::{
        modifiers,
    };
    use ink_storage::{
        collections::{
            HashMap as StorageHashMap,
        },
    };
    use ink_prelude::{
        string::{String},
        vec::Vec,
    };

    #[ink(storage)]
    #[derive(Default, PSP1155Storage, PSP1155MetadataStorage, OwnableStorage, IPSP1155, IOwnable)]
    pub struct PSP1155Struct {}

    impl PSP1155Struct {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            let caller = instance.env().caller();
            instance._init_with_owner(caller);
            instance
        }

        #[inline]
        fn only_owner(&self) {
            assert_eq!(self._owner(), &self.env().caller(), "{}", OwnableError::CallerIsNotOwner.as_ref());
        }
    }

    impl Ownable for PSP1155Struct {}
    impl PSP1155 for PSP1155Struct {}

    impl PSP1155Mint for PSP1155Struct {}
    impl IPSP1155Mint for PSP1155Struct {
        #[ink(message)]
        fn mint(&mut self, to: AccountId, id: Id, amount: Balance) {
            self.only_owner();
            PSP1155Mint::mint(self, to, id, amount);
        }

        #[ink(message)]
        fn burn(&mut self, from: AccountId, id: Id, amount: Balance) {
            self.only_owner();
            PSP1155Mint::burn(self, from, id, amount);
        }
    }

    impl PSP1155Metadata for PSP1155Struct {}
    impl IPSP1155MetadataURI for PSP1155Struct {
        #[ink(message)]
        fn uri(&self, _id: Id) -> Option<String> {
            self._uri().clone()
        }
    }
}