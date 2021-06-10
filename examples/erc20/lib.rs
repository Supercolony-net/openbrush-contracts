#![cfg_attr(not(feature = "std"), no_std)]

#[macros::contract]
pub mod my_erc20 {
    use erc20::{
        traits::{ IErc20, Erc20Error },
        impls::{ Erc20Storage, Erc20Internal, Erc20 },
    };
    use ink_prelude::{
        string::{
            String,
            ToString,
        }
    };
    use ink_storage::{
        collections::{
            HashMap as StorageHashMap,
        }, Lazy
    };
    use utils::{
        traits::{InkStorage},
        iml_getters,
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
    #[derive(Default)]
    pub struct MyErc20 {
        total_supply: Lazy<Balance>,
        balances: StorageHashMap<AccountId, Balance>,
        allowances: StorageHashMap<(AccountId, AccountId), Balance>,
        name: Lazy<Option<String>>,
        symbol: Lazy<Option<String>>,
        decimal: Lazy<u8>,

        // fields for hater logic
        hated_account: AccountId,
    }

    impl InkStorage for MyErc20 {}
    impl Erc20Storage for MyErc20 {
        iml_getters!(total_supply, _supply, _supply_mut, Lazy<Balance>);
        iml_getters!(balances, _balances, _balances_mut, StorageHashMap<AccountId, Balance>);
        iml_getters!(allowances, _allowances, _allowances_mut, StorageHashMap<(AccountId, AccountId), Balance>);
        iml_getters!(name, _name, _name_mut, Lazy<Option<String>>);
        iml_getters!(symbol, _symbol, _symbol_mut, Lazy<Option<String>>);
        iml_getters!(decimal, _decimals, _decimals_mut, Lazy<u8>);
    }

    // Erc20 has additional trait Erc20Internal which contains internal methods which is used for implementation of Erc20 trait.
    // You also can override them. Methods which emit events is not defined in Erc20Internal, so you MUST define them here by self.
    impl Erc20Internal for MyErc20 {
        fn _emit_transfer_event(&self, _from: Option<AccountId>, _to: Option<AccountId>, _amount: Balance) {
            self.env().emit_event(Transfer {
                from: _from,
                to: _to,
                value: _amount,
            });
        }

        fn _emit_approval_event(&self, _owner: AccountId, _spender: AccountId, _amount: Balance) {
            self.env().emit_event(Approval {
                owner: _owner,
                spender: _spender,
                value: _amount,
            });
        }

        // Let's override method to reject transactions to bad account
        fn _before_token_transfer(&mut self, _from: AccountId, _to: AccountId, _amount: Balance) -> Result<(), Erc20Error> {
            if _to == self.hated_account {
                return Err(Erc20Error::Unknown("I hate this account!".to_string()))
            }
            Ok(())
        }
    }
    impl Erc20 for MyErc20 {}

    make_trait!(MyErc20, IErc20);

    impl MyErc20 {
        #[ink(constructor)]
        pub fn new(_total_supply: Balance, name: Option<String>, symbol: Option<String>, decimal: u8) -> Self {
            let mut instance = Self::_empty();
            *instance._name_mut() = Lazy::new(name);
            *instance._symbol_mut() = Lazy::new(symbol);
            instance.set_decimals(decimal);
            instance.mint(instance.env().caller(), _total_supply).expect("Can't mint tokens");
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
