#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod my_psp1155 {
    use ink_prelude::vec::Vec;
    use ink_storage::collections::HashMap as StorageHashMap;
    use psp1155::{
        extensions::mintable::*,
        traits::*,
    };

    #[derive(Default, PSP1155Storage)]
    #[ink(storage)]
    pub struct MyPSP1155 {
        #[PSP1155StorageField]
        psp1155: PSP1155Data,
        registered_ids: StorageHashMap<Id, bool>,
    }

    impl PSP1155 for MyPSP1155 {}

    impl PSP1155Mintable for MyPSP1155 {}

    impl MyPSP1155 {
        /// contract constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        // a method which registers new token type
        #[ink(message)]
        pub fn add_type(&mut self, id: Id) {
            self.registered_ids.insert(id, true);
        }

        /// Mint method which mints `amount` of token type `id`
        ///
        /// `id` must be registered
        #[ink(message)]
        pub fn mint_tokens(&mut self, id: Id, amount: Balance) -> Result<(), PSP1155Error> {
            assert!(*self.registered_ids.get(&id).unwrap_or(&false));
            self.mint(id, amount)
        }
    }
}
