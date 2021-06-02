#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
#[ink::contract]
pub mod my_erc20 {
    use erc20::{
        traits::{ IErc20, Erc20Error },
        impls::*,
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
    impl IErc20 for MyErc20 {
        #[ink(message)]
        fn token_name(&self) -> Option<String> {
            self._token_name()
        }

        #[ink(message)]
        fn token_symbol(&self) -> Option<String> {
            self._token_symbol()
        }

        #[ink(message)]
        fn token_decimals(&self) -> u8 {
            self._token_decimals()
        }

        #[ink(message)]
        fn total_supply(&self) -> Balance {
            self._total_supply()
        }

        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> Balance {
            self._balance_of(owner)
        }

        #[ink(message)]
        fn transfer(&mut self, to: AccountId, value: Balance) -> Result<(), Erc20Error> {
            panic_on_error(self._transfer(to, value))
        }

        #[ink(message)]
        fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self._allowance(owner, spender)
        }

        #[ink(message)]
        fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<(), Erc20Error> {
            panic_on_error(self._transfer_from(from, to, value))
        }

        #[ink(message)]
        fn approve(&mut self, spender: AccountId, value: Balance) -> Result<(), Erc20Error> {
            panic_on_error(self._approve(spender, value))
        }

        #[ink(message)]
        fn increase_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), Erc20Error> {
            panic_on_error(self._increase_allowance(spender, delta_value))
        }

        #[ink(message)]
        fn decrease_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), Erc20Error> {
            panic_on_error(self._decrease_allowance(spender, delta_value))
        }
    }

    impl MyErc20 {
        #[ink(constructor)]
        pub fn new(_total_supply: Balance, name: Option<String>, symbol: Option<String>, decimal: u8) -> Self {
            let mut instance = Self::_empty();
            *instance._name_mut() = Lazy::new(name);
            *instance._symbol_mut() = Lazy::new(symbol);
            instance._set_decimals(decimal);
            instance._mint(instance.env().caller(), _total_supply).expect("Can't mint tokens");
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

    // TODO: ink! doesn't revert transactions if you returned error from the public method,
    // so let's do it manually for now. https://github.com/paritytech/ink/issues/641
    fn panic_on_error<T, E>(result: Result<T, E>) -> Result<T, E> {
        match result {
            Err(_) => panic!("Got error during execution"),
            Ok(ok) => Ok(ok),
        }
    }
}
