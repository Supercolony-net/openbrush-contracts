## Overview
This example shows how you can reuse the implementation of
[psp20](contracts/token/psp20) token(by the same way you can reuse
[psp721](contracts/token/psp721) and [psp1155](contracts/token/psp1155)).
Also, this example shows how you can customize the logic, for example, to not allow transfer tokens to `hated_account`.

## Steps
1. You need to include `psp20` and `brush` in cargo file.
```markdown
[dependencies]
...

psp20 = { version = "0.3.0-rc1", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false, features = ["ink-as-dependency"] }
brush = { version = "0.3.0-rc1", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }

[features]
default = ["std"]
std = [
 ...
   
   "psp20/std",
   "brush/std",
]
```
2. To declare the contract you need to use `brush::contract` macro instead of `ink::contract`.
   Import **everything** from `psp20` trait module.
```rust
#[brush::contract]
pub mod my_psp20 {
   use psp20::traits::*;
```
3. Declare storage struct and derive `PSP20Storage`trait. Deriving this trait 
   will add required fields to your structure for implementation of according trait. 
   Your structure must implement `PSP20Storage` if you want to use the
   default implementation of `IPSP20`.

```rust
#[ink(storage)]
#[derive(Default, PSP20Storage)]
pub struct MyPSP20 {}
```
4. After that you can inherit implementation of `IPSP20` trait.
   You can customize(override) some methods there.
```rust
impl PSP20 for MyPSP20 {}
```
5. Now you only need to define constructor and your basic version of `PSP20` contract is ready.
```rust
impl MyPSP20 {
   #[ink(constructor)]
   pub fn new(_total_supply: Balance, name: Option<String>, symbol: Option<String>, decimal: u8) -> Self {
      let mut instance = Self::default();
      *instance._name_mut() = Lazy::new(name);
      *instance._symbol_mut() = Lazy::new(symbol);
      *instance._decimals_mut() = Lazy::new(decimal);
      instance._mint(instance.env().caller(), _total_supply);
      instance
   }
}
```
6. Let's customize it. It will contain two public methods `set_hated_account` and `get_hated_account`. 
   Also we will override `_before_token_transfer` method in `IPSP20` implementation.
   And we will add a new field to structure - `hated_account: AccountId`
```rust
#[ink(storage)]
#[derive(Default, PSP20Storage)]
pub struct MyPSP20 {
   // fields for hater logic
   hated_account: AccountId,
}
impl IPSP20 for MyPSP20 {
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
      *instance._decimals_mut() = Lazy::new(decimal);
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
```
