#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp1155 {
    use ink_prelude::vec;
    use ink_storage::traits::SpreadAllocate;
    use openbrush::contracts::psp1155::extensions::burnable::*;

    #[derive(Default, SpreadAllocate, PSP1155Storage)]
    #[ink(storage)]
    pub struct MyPSP1155 {
        #[PSP1155StorageField]
        psp1155: PSP1155Data,
    }

    impl PSP1155 for MyPSP1155 {}

    impl PSP1155Burnable for MyPSP1155 {}

    impl MyPSP1155 {
        /// contract constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                let caller = Self::env().caller();
                instance
                    ._mint_to(caller, vec![([0; 32], 1)])
                    .expect("Should mint token");
                let mut id = [0; 32];
                id[0] = 1;
                instance._mint_to(caller, vec![(id, 20)]).expect("Should mint token");
            })
        }
    }
}
