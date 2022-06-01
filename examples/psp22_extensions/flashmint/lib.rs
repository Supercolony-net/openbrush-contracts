#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22_flashmint {
    use ink_storage::traits::SpreadAllocate;
    use openbrush::contracts::psp22::extensions::flashmint::*;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, PSP22Storage)]
    pub struct MyPSP22FlashMint {
        #[PSP22StorageField]
        psp22: PSP22Data,
    }

    impl PSP22 for MyPSP22FlashMint {}

    impl FlashLender for MyPSP22FlashMint {}

    // To override an internal method from OpenBrush implementation
    // you need to override that in the `PSP22FlashLenderInternal` trait
    impl PSP22FlashLenderInternal for MyPSP22FlashMint {
        /// Override `get_fee` function to add 1% fee to the borrowed `amount`
        fn _get_fee(&self, amount: Balance) -> Balance {
            amount / 100
        }
    }

    impl MyPSP22FlashMint {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                assert!(instance._mint(instance.env().caller(), total_supply).is_ok());
            })
        }
    }
}
