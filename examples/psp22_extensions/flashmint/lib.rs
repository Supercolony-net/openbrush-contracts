#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod my_psp22_flashmint {
    use psp22::{
        extensions::flashmint::*,
        traits::*,
    };

    #[ink(storage)]
    #[derive(Default, PSP22Storage)]
    pub struct MyPSP22FlashMint {
        #[PSP22StorageField]
        psp22: PSP22Data,
    }

    impl PSP22 for MyPSP22FlashMint {}

    impl PSP22FlashMint for MyPSP22FlashMint {
        // we will add 1% fee to the amount
        fn get_fee(&mut self, amount: Balance) -> Balance {
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
