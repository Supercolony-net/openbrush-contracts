#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod my_psp1155 {
    use brush::contracts::psp1155::extensions::mintable::*;
    use ink_storage::traits::SpreadAllocate;

    #[derive(Default, SpreadAllocate, PSP1155Storage)]
    #[ink(storage)]
    pub struct MyPSP1155 {
        #[PSP1155StorageField]
        psp1155: PSP1155Data,
    }

    impl PSP1155 for MyPSP1155 {}

    impl PSP1155Mintable for MyPSP1155 {}

    impl MyPSP1155 {
        /// contract constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self| {})
        }
    }
}
