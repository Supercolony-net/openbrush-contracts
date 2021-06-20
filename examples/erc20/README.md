## Overview
This example shows how you can reuse the implementation of
[erc20](contracts/token/erc20) token(by the same way you can reuse
[erc721](contracts/token/erc721) and [erc1155](contracts/token/erc1155)).
Also, this example shows how you can customize the logic, for example, to not allow transfer tokens to `hated_account`.

## Steps
1. You need to include `erc20` and `brush` in cargo file.
```markdown
[dependencies]
...

erc20 = { version = "0.2.0-rc1", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false, features = ["ink-as-dependency"] }
brush = { version = "0.2.0-rc1", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }

[features]
default = ["std"]
std = [
 ...
   
   "erc20/std",
   "brush/std",
]
```
2. To declare the contract you need to use `brush::contract` macro instead of `ink::contract`.
Import traits, errors, macros and structs which you want to use.
```rust
#[brush::contract]
pub mod my_erc20 {
    use erc20::{
        traits::{ IErc20, Erc20Error },
        impls::{ Erc20Storage, Erc20 },
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
3. Declare storage struct that will contain all fields for 
`Erc20Storage` trait.
Declare events(example of events you can find in tests of [Erc20](contracts/token/erc20/impls.rs))

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
// Inheritance of Erc20 requires you to implement methods for event dispatching
impl Erc20 for MyErc20 {
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
```
6. Now you have all basic logic of `Erc20` on rust level.
   But all methods are internal now(it means that anyone can't call these methods from outside of contract).
   If you want to make them external you MUST implement `IErc20` trait.
   Library provides macro `impl_trait` that will generate external implementation of all methods from `IErc20` trait.
   Macro will call the methods with the same name from `Erc20` trait.
```rust
impl_trait!(MyErc20, IErc20(Erc20));
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
8. Let's customize it. It will contain two public methods `set_hated_account` and `get_hated_account`.
Also we will override `_before_token_transfer` method in `Erc20` implementation.
```rust
// Inheritance of Erc20 requires you to implement methods for event dispatching
impl Erc20 for MyErc20 {
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
    fn _before_token_transfer(&mut self, _from: AccountId, _to: AccountId, _amount: Balance) {
        assert!(_to != self.hated_account, "{}", Erc20Error::Unknown("I hate this account!".to_string()).as_ref());
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
