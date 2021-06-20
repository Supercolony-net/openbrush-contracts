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

erc20 = { version = "0.3.0-rc1", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false, features = ["ink-as-dependency"] }
brush = { version = "0.3.0-rc1", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }

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
```
3. Declare storage struct and derive `Erc20Storage`trait. Deriving this trait 
   will add required fields to your structure for implementation of according trait. 
   Your structure must implement `Erc20Storage` if you want to use the
   default implementation of `Erc20`.

```rust
#[ink(storage)]
#[derive(Default, Erc20Storage)]
pub struct MyErc20 {}
```
4. After that you can inherit implementation of `Erc20` trait.
   You can customize(override) some methods there.
```rust
impl InkStorage for MyErc20 {}
impl Erc20 for MyErc20 {}
```
5. Now you have all basic logic of `Erc20` on rust level.
   But all methods are internal now(it means that anyone can't call these methods from outside of contract).
   If you want to make them external you MUST derive `IErc20` trait.
   Deriving of this trait will generate external implementation of all methods from `IErc20`.
   Macro will call the methods with the same name from `Erc20` trait.
```rust
#[ink(storage)]
#[derive(Default, Erc20Storage, IErc20)]
pub struct MyErc20 {}
```
6. Now you only need to define constructor and your basic version of `Erc20` contract is ready.
```rust
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
}
```
7. Let's customize it. It will contain two public methods `set_hated_account` and `get_hated_account`. 
   Also we will override `_before_token_transfer` method in `Erc20` implementation.
   And we will add a new field to structure - `hated_account: AccountId`
```rust
#[ink(storage)]
#[derive(Default, Erc20Storage, IErc20)]
pub struct MyErc20 {
   // fields for hater logic
   hated_account: AccountId,
}
...
impl Erc20 for MyErc20 {
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
