#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod my_psp35 {
    use brush::contracts::psp35::extensions::batch::*;
    use ink_prelude::vec;
    use ink_storage::traits::SpreadAllocate;

    #[derive(Default, SpreadAllocate, PSP35Storage)]
    #[ink(storage)]
    pub struct MyPSP35 {
        #[PSP35StorageField]
        psp35: PSP35Data,
    }

    impl PSP35 for MyPSP35 {}

    impl PSP35Batch for MyPSP35 {}

    impl MyPSP35 {
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
