#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod my_psp22_flashmint {
    use brush::contracts::psp22::extensions::flashmint::*;

    #[ink(storage)]
    #[derive(Default, PSP22Storage)]
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
        fn _get_fee(&mut self, amount: Balance) -> Balance {
            amount / 100
        }
    }

    impl MyPSP22FlashMint {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self::default();
            assert!(instance._mint(instance.env().caller(), total_supply).is_ok());
            instance
        }
    }
}
