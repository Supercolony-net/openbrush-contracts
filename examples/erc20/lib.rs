#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod my_erc20 {
    use erc20::{
        traits::{ IErc20, Erc20Error },
        impls::{ Erc20Storage, Erc20, StorageHashMap, Lazy, String },
    };
    use ink_prelude::{
        string::{
            ToString,
        }
    };
    use brush::{
        traits::{InkStorage},
    };
    use ink_lang::{Env, EmitEvent};

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    /// Event emitted when an approval occurs that `spender` is allowed to withdraw
    /// up to the amount of `value` tokens from `owner`.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    #[ink(storage)]
    #[derive(Default, Erc20Storage, IErc20)]
    pub struct MyErc20 {
        // fields for hater logic
        hated_account: AccountId,
    }

    // InkStorage is a utils trait required by any Storage trait
    impl InkStorage for MyErc20 {}

    // Inheritance of Erc20 requires you to implement methods for event dispatching
    impl Erc20 for MyErc20 {
        // Let's override method to reject transactions to bad account
        fn _before_token_transfer(&mut self, _from: AccountId, _to: AccountId, _amount: Balance) {
            assert!(_to != self.hated_account, "{}", Erc20Error::Unknown("I hate this account!".to_string()).as_ref());
        }
    }

    impl MyErc20 {
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
