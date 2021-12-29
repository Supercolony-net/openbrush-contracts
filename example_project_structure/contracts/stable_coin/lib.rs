#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

/// This is a simple `PSP-22` which will be used as a stable coin and a collateral token in our lending contract
#[brush::contract]
pub mod token {
    use brush::contracts::psp22::extensions::metadata::*;
    use ink_prelude::string::String;
    use lending_project::traits::stable_coin::*;

    /// Define the storage for PSP22 data and Metadata data
    #[ink(storage)]
    #[derive(Default, PSP22Storage, PSP22MetadataStorage)]
    pub struct StableCoinContract {
        #[PSP22StorageField]
        psp22: PSP22Data,
        #[PSP22MetadataStorageField]
        metadata: PSP22MetadataData,
    }

    /// implement PSP22 Trait for our coin
    impl PSP22 for StableCoinContract {}

    /// implement PSP22Metadata Trait for our coin
    impl PSP22Metadata for StableCoinContract {}

    // It forces the compiler to check that you implemented all super traits
    impl StableCoin for StableCoinContract {}

    impl StableCoinContract {
        /// constructor with name and symbol
        #[ink(constructor)]
        pub fn new(name: Option<String>, symbol: Option<String>) -> Self {
            let mut instance = Self::default();
            instance.metadata.name = name;
            instance.metadata.symbol = symbol;
            instance.metadata.decimals = 18;
            let total_supply = 1_000_000 * 10_u128.pow(18);
            assert!(instance._mint(instance.env().caller(), total_supply).is_ok());
            instance
        }
    }
}
