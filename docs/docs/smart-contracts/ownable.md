---
sidebar_position: 3
title: Ownable & PSP1155
---

This example shows how you can use the implementation of
[access-control](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/access/ownable) and
[psp1155](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp1155) together to provide `only owner` rights to mint and burn tokens.

## Step 1: Include dependencies

Include dependencies to `psp1155`, `ownable` and `brush` in the cargo file.

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
psp1155 = { tag = "v1.0.0", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }
ownable = { tag = "v1.0.0", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }
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
   "psp1155/std",
   "ownable/std",
   "brush/std",
]
```

## Step 2: Add imports

Replace `ink::contract` macro by `brush::contract`.
Import **everything** from `psp1155::traits` and `ownable::traits`.

```rust
#[brush::contract]
pub mod ownable {
   use psp1155::traits::*;
   use ownable::traits::*;
   use brush::modifiers;
   use ink_prelude::vec::Vec;
```

## Step 3: Define storage

Declare storage struct and declare the fields related to `PSP1155Storage` and `OwnableStorage`
traits. Then you need to derive `PSP1155Storage` and `OwnableStorage` traits and mark corresponding fields
with `#[PSP1155StorageField]` and `#[OwnableStorageField]` attributes. Deriving these traits allows you to reuse the
default implementation of `IPSP1155` and `Ownable`.

```rust
#[ink(storage)]
#[derive(Default, PSP1155Storage, OwnableStorage)]
pub struct PSP1155Struct {
    #[PSP1155StorageField]
    psp1155: PSP1155Data,
    #[OwnableStorageField]
    ownale: OwnableData,
}
```

## Step 4: Inherit logic

Inherit implementations of `IPSP1155` and `Ownable` traits. You can customize (override) methods in this `impl` block.

```rust
impl Ownable for PSP1155Struct {}
impl IPSP1155 for PSP1155Struct {}
```

## Step 5: Define constructor

Define constructor and initialize the owner with the contract initiator. Your basic version of `IPSP1155` contract is ready!

```rust
impl PSP1155Struct {
    #[ink(constructor)]
    pub fn new() -> Self {
        let mut instance = Self::default();
        let caller = instance.env().caller();
        instance._init_with_owner(caller);
        instance
    }
}
```

## Step 6: Customize your contract

Customize it by adding ownable logic. We will implement `IPSP1155Mint` trait. Modifier `only_owner` will call the function for us which verifies that
caller is the owner.

```rust
impl IPSP1155Mint for PSP1155Struct {
   #[ink(message)]
   #[modifiers(only_owner)]
   fn mint(&mut self, to: AccountId, id: Id, amount: Balance) {
      self._mint(to, id, amount);
   }

   #[ink(message)]
   #[modifiers(only_owner)]
   fn burn(&mut self, from: AccountId, id: Id, amount: Balance) {
      self._burn(from, id, amount);
   }
}
```
