#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod my_psp22 {
    use psp22::traits::*;
    use ink_storage::Lazy;
    use ink_prelude::{string::String, vec::Vec};

    #[ink(storage)]
    #[derive(Default, PSP22Storage, PSP22MetadataStorage)]
    pub struct MyPSP22 {
        #[PSP22StorageField]
        psp22: PSP22Data,
        #[PSP22MetadataStorageField]
        metadata: PSP22MetadataData,
        // fields for hater logic
        hated_account: AccountId,
    }

    impl PSP22 for MyPSP22 {
        // Let's override method to reject transactions to bad account
        fn _before_token_transfer(&mut self, _from: AccountId, _to: AccountId, _amount: Balance) {
            assert!(_to != self.hated_account, "{}", PSP22Error::Custom(String::from("I hate this account!")).as_ref());
        }
    }

    impl PSP22Metadata for MyPSP22 {}

    impl MyPSP22 {
        #[ink(constructor)]
        pub fn new(_total_supply: Balance, name: Option<String>, symbol: Option<String>, decimal: u8) -> Self {
            let mut instance = Self::default();
            Lazy::set(&mut instance.metadata.name, name);
            Lazy::set(&mut instance.metadata.symbol,symbol);
            Lazy::set(&mut instance.metadata.decimals,decimal);
            instance._mint(instance.env().caller(), _total_supply);
            instance
        }

        #[ink(message)]
        pub fn set_hated_account(&mut self, hated: AccountId) {
            self.hated_account = hated;
        }

        #[ink(message)]
        pub fn get_hated_account(&self) -> AccountId {
            self.hated_account.clone()
        }
    }
}
