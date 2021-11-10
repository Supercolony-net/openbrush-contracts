#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod my_psp1155 {
    use ink_prelude::{
        string::String,
        vec,
    };
    use ink_storage::collections::HashMap as StorageHashMap;
    use psp1155::traits::*;

    #[derive(Default, PSP1155Storage)]
    #[ink(storage)]
    pub struct MyPSP1155 {
        #[PSP1155StorageField]
        psp1155: PSP1155Data,
        denied_ids: StorageHashMap<Id, ()>,
    }

    impl PSP1155 for MyPSP1155 {}

    impl MyPSP1155 {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn deny(&mut self, id: Id) {
            self.denied_ids.insert(id, ());
        }

        #[ink(message)]
        pub fn mint_tokens(&mut self, id: Id, amount: Balance) -> Result<(), PSP1155Error> {
            if self.denied_ids.get(&id).is_some() {
                return Err(PSP1155Error::Custom(String::from("Id is denied")))
            }
            self._mint_to(Self::env().caller(), vec![(id, amount)])
        }
    }
}
