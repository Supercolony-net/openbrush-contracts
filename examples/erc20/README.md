## Overview
This example shows how you can reuse the implementation of
[erc20](contracts/token/erc20) token(by the same way you can reuse
[erc721](contracts/token/erc721) and [erc1155](contracts/token/erc1155)).
Also, this example shows how you can customize the logic, for example, to not allow transfer tokens to `hated_account`.

## Steps
1. You need to include `erc20` and `utils` in cargo file.
```markdown
[dependencies]
...

erc20 = { version = "0.1.0-rc1", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false, features = ["ink-as-dependency"] }
utils = { version = "0.1.0-rc1", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }

[features]
default = ["std"]
std = [
 ...
   
   "erc20/std",
   "brush/std",
]
```
2. Import according: traits, errors, macros and structs.
```rust
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
use brush::{
    traits::{InkStorage},
    iml_getters,
};
use ink_lang::{Env, EmitEvent};
```
3. Define storage struct that will contains all fields for 
`Erc20Storage` trait. Define events(example of events you can find in tests of [Erc20](contracts/token/erc20/impls.rs))

```rust
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
```
4. Implement storage trait by using `iml_getters` macro.
```rust
impl InkStorage for MyErc20 {}
impl Erc20Storage for MyErc20 {
    iml_getters!(total_supply, _supply, _supply_mut, Lazy<Balance>);
    iml_getters!(balances, _balances, _balances_mut, StorageHashMap<AccountId, Balance>);
    iml_getters!(allowances, _allowances, _allowances_mut, StorageHashMap<(AccountId, AccountId), Balance>);
    iml_getters!(name, _name, _name_mut, Lazy<Option<String>>);
    iml_getters!(symbol, _symbol, _symbol_mut, Lazy<Option<String>>);
    iml_getters!(decimal, _decimals, _decimals_mut, Lazy<u8>);
}
```
5. After that you can inherit implementation of `Erc20`.
```rust
// Erc20 has additional trait Erc20Internal which contains internal methods which is used for implementation of Erc20 trait.
// You also can override them. Methods which emit events is not defined in Erc20Internal, so you MUST define them here by self.
impl Erc20Internal for MyErc20 {
    fn emit_transfer_event(&self, _from: Option<AccountId>, _to: Option<AccountId>, _amount: Balance) {
        self.env().emit_event(Transfer {
            from: _from,
            to: _to,
            value: _amount,
        });
    }

    fn emit_approval_event(&self, _owner: AccountId, _spender: AccountId, _amount: Balance) {
        self.env().emit_event(Approval {
            owner: _owner,
            spender: _spender,
            value: _amount,
        });
    }
}
impl Erc20 for MyErc20 {}
```
6. Now you have all basic logic of `Erc20` on rust level.
All methods are private now. If you want to make them public you MUST implement `IErc20` trait.

```rust
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
        self._transfer(to, value)
    }

    #[ink(message)]
    fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
        self._allowance(owner, spender)
    }

    #[ink(message)]
    fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<(), Erc20Error> {
        self._transfer_from(from, to, value)
    }

    #[ink(message)]
    fn approve(&mut self, spender: AccountId, value: Balance) -> Result<(), Erc20Error> {
        self._approve(spender, value)
    }

    #[ink(message)]
    fn increase_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), Erc20Error> {
        self._increase_allowance(spender, delta_value)
    }

    #[ink(message)]
    fn decrease_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), Erc20Error> {
        self._decrease_allowance(spender, delta_value)
    }
}
```
7. Now you only need to define constructor and your basic version of `Erc20` contract is ready.
```rust
impl MyErc20 {
    #[ink(constructor)]
    pub fn new(_total_supply: Balance, name: Option<String>, symbol: Option<String>, decimal: u8) -> Self {
        let mut instance = Self::_empty();
        *instance._name_mut() = Lazy::new(name);
        *instance._symbol_mut() = Lazy::new(symbol);
        instance.set_decimals(decimal);
        instance._mint(instance.env().caller(), _total_supply).expect("Can't mint tokens");
        instance
    }
}
```
8. Let's customize it. It will contains two public methods `set_hated_account` and `get_hated_account`.
Also we will override `_before_token_transfer` method in `Erc20Internal` implementation.
```rust
// Erc20 has additional trait Erc20Internal which contains internal methods which is used for implementation of Erc20 trait.
// You also can override them. Methods which emit events is not defined in Erc20Internal, so you MUST define them here by self.
impl Erc20Internal for MyErc20 {
    fn emit_transfer_event(&self, _from: Option<AccountId>, _to: Option<AccountId>, _amount: Balance) {
        self.env().emit_event(Transfer {
            from: _from,
            to: _to,
            value: _amount,
        });
    }

    fn emit_approval_event(&self, _owner: AccountId, _spender: AccountId, _amount: Balance) {
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

impl MyErc20 {
    #[ink(constructor)]
    pub fn new(_total_supply: Balance, name: Option<String>, symbol: Option<String>, decimal: u8) -> Self {
        let mut instance = Self::_empty();
        *instance._name_mut() = Lazy::new(name);
        *instance._symbol_mut() = Lazy::new(symbol);
        instance.set_decimals(decimal);
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
```
9. The last step(but it is optional) is to panic on error in public methods to force revert of transaction.
For that let's call wrapper function in public methods.
```rust
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
// TODO: ink! doesn't revert transactions if you returned error from the public method,
// so let's do it manually for now. https://github.com/paritytech/ink/issues/641
fn panic_on_error<T, E>(result: Result<T, E>) -> Result<T, E> {
    match result {
        Err(_) => panic!("Got error during execution"),
        Ok(ok) => Ok(ok),
    }
}
```
