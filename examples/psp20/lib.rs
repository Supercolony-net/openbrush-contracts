#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod my_psp20 {
    use psp20::{
        traits::{ IPSP20, PSP20Error },
        impls::{ PSP20Storage, PSP20, StorageHashMap, Lazy, String },
    };
    use brush::{
        traits::{InkStorage},
    };

    #[ink(storage)]
    #[derive(Default, PSP20Storage, IPSP20)]
    pub struct MyPSP20 {
        // fields for hater logic
        hated_account: AccountId,
    }

    // InkStorage is a utils trait required by any Storage trait
    impl InkStorage for MyPSP20 {}
    impl PSP20 for MyPSP20 {
        // Let's override method to reject transactions to bad account
        fn _before_token_transfer(&mut self, _from: AccountId, _to: AccountId, _amount: Balance) {
            assert!(_to != self.hated_account, "{}", PSP20Error::Unknown("I hate this account!").as_ref());
        }
    }

    impl MyPSP20 {
        #[ink(constructor)]
        pub fn new(_total_supply: Balance, name: Option<String>, symbol: Option<String>, decimal: u8) -> Self {
            let mut instance = Self::default();
            *instance._name_mut() = Lazy::new(name);
            *instance._symbol_mut() = Lazy::new(symbol);
            instance.set_decimals(decimal);
            instance.mint(instance.env().caller(), _total_supply);
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
