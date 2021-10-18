#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod my_psp1155 {
    use ink_storage::collections::HashMap as StorageHashMap;
    use psp1155::{
        extensions::{
            burnable::*,
            mintable::*,
        },
        traits::*,
    };

    #[derive(Default, PSP1155Storage, PSP1155MetadataStorage)]
    #[ink(storage)]
    pub struct MyPSP1155 {
        #[PSP1155StorageField]
        psp1155: PSP1155Data,
        #[PSP1155MetadataStorageField]
        metadata: PSP1155MetadataData,
        unique_ids: u8,
        registered_ids: StorageHashMap<Id, bool>,
    }

    impl PSP1155 for MyPSP1155 {}

    impl PSP1155Burnable for MyPSP1155 {}

    impl PSP1155Mintable for MyPSP1155 {}

    impl MyPSP1155 {
        /// contract constructor
        #[ink(constructor)]
        pub fn new(name: Option<String>, symbol: Option<String>) -> Self {
            let mut instance = Self::default();
            instance.metadata.name = name;
            instance.metadata.symbol = symbol;
            instance
        }

        // a method which registers new token type
        #[ink(message)]
        pub fn add_type(&mut self) {
            self.registered_ids.insert([self.unique_ids; 32], true);
            self.unique_ids += 1;
        }

        /// Mint method which mints `amount` of token type `id`
        ///
        /// `id` must be registered
        #[ink(message)]
        pub fn mint_tokens(&mut self, id: Id, amount: Balance) {
            assert!(*self.registered_ids.get(&id).unwrap_or(&false));
            self.mint(id, amount);
        }
    }
}
