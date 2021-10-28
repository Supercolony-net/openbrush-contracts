#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod my_psp1155 {
    use ink_prelude::vec::Vec;
    use ink_storage::collections::HashMap as StorageHashMap;
    use psp1155::traits::*;

    #[derive(Default, PSP1155Storage)]
    #[ink(storage)]
    pub struct MyPSP1155 {
        #[PSP1155StorageField]
        psp1155: PSP1155Data,
        registered_ids: StorageHashMap<Id, bool>,
    }

    impl PSP1155 for MyPSP1155 {}

    impl MyPSP1155 {
        /// contract constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            let caller = Self::env().caller();
            match instance._mint(caller, [0; 32], 1) {
                Ok(result) => result,
                Err(e) => panic!("{}", e.as_ref()),
            }
            let mut id = [0; 32];
            id[0] = 1;
            match instance._mint(caller, id, 20) {
                Ok(result) => result,
                Err(e) => panic!("{}", e.as_ref()),
            }
            instance
        }
    }
}
