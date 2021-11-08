#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod my_psp22 {
    use ink_prelude::{
        string::String,
        vec::Vec,
    };
    use ink_storage::Lazy;
    use psp22::{
        extensions::metadata::*,
        traits::*,
    };

    #[ink(storage)]
    #[derive(Default, PSP22Storage, PSP22MetadataStorage)]
    pub struct MyPSP22 {
        #[PSP22StorageField]
        psp22: PSP22Data,
        #[PSP22MetadataStorageField]
        metadata: PSP22MetadataData,
    }

    impl PSP22 for MyPSP22 {}

    impl PSP22Metadata for MyPSP22 {}

    impl MyPSP22 {
        #[ink(constructor)]
        pub fn new(_total_supply: Balance, name: Option<String>, symbol: Option<String>, decimal: u8) -> Self {
            let mut instance = Self::default();
            Lazy::set(&mut instance.metadata.name, name);
            Lazy::set(&mut instance.metadata.symbol, symbol);
            Lazy::set(&mut instance.metadata.decimals, decimal);
            assert!(instance._mint(instance.env().caller(), _total_supply).is_ok());
            instance
        }
    }
}
