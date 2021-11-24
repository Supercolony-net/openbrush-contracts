#![cfg_attr(not(feature = "std"), no_std)]

/// This contract will represent a stable coin which will be lended and borrowed inside our contract
#[brush::contract]
pub mod stable_coin {
    use ink_prelude::string::String;
    use ink_storage::Lazy;
    use psp22::{
        extensions::metadata::*,
        traits::*,
    };

    /// Define the storage for PSP22 data and Metadata data
    #[ink(storage)]
    #[derive(Default, PSP22Storage, PSP22MetadataStorage)]
    pub struct StableCoin {
        #[PSP22StorageField]
        psp22: PSP22Data,
        #[PSP22MetadataStorageField]
        metadata: PSP22MetadataData,
    }

    /// implement PSP22 Trait for our coin
    impl PSP22 for StableCoin {}

    /// implement PSP22Metadata Trait for our coin
    impl PSP22Metadata for StableCoin {}

    impl StableCoin {
        /// Define constructor where we mint the initial amount to the deployer and set the metadata values
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            let total_supply = 1_000_000 * 10_u128.pow(18);
            Lazy::set(&mut instance.metadata.name, Some(String::from("Stable Coin")));
            Lazy::set(&mut instance.metadata.symbol, Some(String::from("SC")));
            Lazy::set(&mut instance.metadata.decimals, 18);
            assert!(instance._mint(instance.env().caller(), total_supply).is_ok());
            instance
        }
    }
}
