## Overview

This example shows how you can reuse the implementation of
[psp22](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp22) token(by the same way you can reuse
[psp721](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp721) and [psp1155](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp1155)). Also, this example shows how you can customize
the logic, for example, to reject transfering tokens to `hated_account`.

## Steps

1. Include dependencies to `psp22` and `brush` in the cargo file.

```markdown
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
psp22 = { tag = "v0.3.0-rc1", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }
brush = { tag = "v0.3.0-rc1", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }

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

2. Replace `ink::contract` macro by `brush::contract`.
   Import **everything** from `psp22::traits`.

```rust
#[brush::contract]
pub mod my_psp22 {
   use psp22::traits::*;
   use ink_storage::Lazy;
   use ink_prelude::{string::String, vec::Vec};
```

3. Declare storage struct and declare the fields related to `PSP22Storage` and `PSP22MetadataStorage`
   traits. Then you need to derive `PSP22Storage` and `PSP22MetadataStorage` traits and mark corresponsing fields
   with `#[PSP22StorageField]` and `#[PSP22MetadataStorageField]` attributes. Deriving these traits allows you to reuse
   the default implementation of `PSP22` and `PSP22Metadata`.

```rust
#[ink(storage)]
#[derive(Default, PSP22Storage, PSP22MetadataStorage)]
pub struct MyPSP22 {
    #[PSP22StorageField]
    psp22: PSP22Data,
    #[PSP22MetadataStorageField]
    metadata: PSP22MetadataData,
}
```

4. Inherit implementations of `PSP22` and `PSP22Metadata` traits. You can customize (override) methods in this `impl` block.

```rust
impl PSP22 for MyPSP22 {}

impl PSP22Metadata for MyPSP22 {}
```

5. Define constructor. Your basic version of `PSP22` contract is ready!

```rust
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
}
```

6. Customize it by adding hated account logic. It will contain two public methods `set_hated_account` and `get_hated_account`. Also we will
   override `_before_token_transfer` method in `PSP22` implementation. And we will add the `hated_account: AccountId` field to the structure.

```rust
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
```
