---
sidebar_position: 1
title: PSP22
---

This example shows how you can reuse the implementation of [PSP22](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp22) token. Also, this example shows how you can customize the logic, for example, to reject transferring tokens to `hated_account`.

## Step 1: Include dependencies

Include dependencies to `psp22` and `brush` in the cargo file.

```toml
[dependencies]
ink_primitives = { tag = "v3.0.0-rc4", git = "https://github.com/Supercolony-net/ink", default-features = false }
ink_metadata = { tag = "v3.0.0-rc4", git = "https://github.com/Supercolony-net/ink", default-features = false, features = ["derive"], optional = true }
ink_env = { tag = "v3.0.0-rc4", git = "https://github.com/Supercolony-net/ink", default-features = false }
ink_storage = { tag = "v3.0.0-rc4", git = "https://github.com/Supercolony-net/ink", default-features = false }
ink_lang = { tag = "v3.0.0-rc4", git = "https://github.com/Supercolony-net/ink", default-features = false }
ink_prelude = { tag = "v3.0.0-rc4", git = "https://github.com/Supercolony-net/ink", default-features = false }

scale = { package = "parity-scale-codec", version = "2.1", default-features = false, features = ["derive"] }
scale-info = { version = "0.6.0", default-features = false, features = ["derive"], optional = true }

# These dependencies
psp22 = { tag = "v1.0.0", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }
brush = { tag = "v1.0.0", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }

[features]
default = ["std"]
std = [
   "ink_primitives/std",
   "ink_metadata",
   "ink_metadata/std",
   "ink_env/std",
   "ink_storage/std",
   "ink_lang/std",
   "scale/std",
   "scale-info",
   "scale-info/std",

   # These dependencies   
   "psp22/std",
   "brush/std",
]
```

## Step 2: Add imports

Replace `ink::contract` macro by `brush::contract`.
Import **everything** from `psp22::traits`.

```rust
#[brush::contract]
pub mod my_psp22 {
   use psp22::traits::*;
   use ink_storage::Lazy;
   use ink_prelude::{string::String, vec::Vec};
```

## Step 3: Define storage

Declare the storage struct and declare the field related to the `PSP22Storage` trait. Then you need to derive the `PSP22Storage` trait and mark the corresponding field with the `#[PSP22StorageField]` attribute. Deriving this trait allows you to reuse the default implementation of `PSP22`.

```rust
#[ink(storage)]
#[derive(Default, PSP22Storage)]
pub struct MyPSP22 {
    #[PSP22StorageField]
    psp22: PSP22Data,
}
```

## Step 4: Inherit logic

Inherit the implementation of `PSP22` trait. You can customize (override) methods in the `impl` block.

```rust
impl PSP22 for MyPSP22 {}
```

## Step 5: Define constructor

Define constructor. Your basic version of `PSP22` contract is ready!

```rust
impl MyPSP22 {
   #[ink(constructor)]
   pub fn new(total_supply: Balance) -> Self {
      let mut instance = Self::default();
      instance._mint(instance.env().caller(), total_supply);
      instance
   }
}
```

## Step 6: Customize your contract

Customize it by adding hated account logic. It will contain two public methods `set_hated_account` and `get_hated_account`. Also we will
override `_before_token_transfer` method in the `PSP22` implementation, and we will add the `hated_account: AccountId` field to the structure.

```rust
#[ink(storage)]
#[derive(Default, PSP22Storage)]
pub struct MyPSP22 {
   #[PSP22StorageField]
   psp22: PSP22Data,
   // fields for hater logic
   hated_account: AccountId,
}

impl PSP22 for MyPSP22 {
   // Let's override method to reject transactions to bad account
   fn _before_token_transfer(&mut self, _from: AccountId, _to: AccountId, _amount: Balance) {
      assert!(_to != self.hated_account, "{}", PSP22Error::Custom(String::from("I hate this account!")).as_ref());
   }
}

impl MyPSP22 {
   #[ink(constructor)]
   pub fn new(total_supply: Balance) -> Self {
      let mut instance = Self::default();
      instance._mint(instance.env().caller(), total_supply);
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


Also you can use extensions for psp22 token:

[PSP22Metadata](/smart-contracts/psp22/extensions/metadata): metadata for PSP22.

[PSP22Mintable](/smart-contracts/psp22/extensions/mintable): creation of new tokens.

[PSP22Burnable](/smart-contracts/psp22/extensions/burnable): destruction of own tokens.
