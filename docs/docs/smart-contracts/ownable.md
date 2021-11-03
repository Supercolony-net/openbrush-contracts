---
sidebar_position: 3
title: Ownable
---

This example shows how you can use the implementation of [ownable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/access/ownable) to provide `only owner` rights for contract's functions.

## Step 1: Include dependencies

Include dependencies to `ownable` and `brush` in the cargo file.

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
   "ownable/std",
   "brush/std",
]
```

## Step 2: Add imports

Replace `ink::contract` macro by `brush::contract`.
Import **everything** from `ownable::traits`.

```rust
#[brush::contract]
pub mod ownable {
   use ownable::traits::*;
   use brush::modifiers;
   use ink_prelude::vec::Vec;
```

## Step 3: Define storage

Declare storage struct and declare the field related to `OwnableStorage` trait. Then you need to derive the `OwnableStorage` trait and mark the corresponding field with the `#[OwnableStorageField]` attribute. Deriving this trait allows you to reuse the default implementation of `Ownable`.

```rust
#[ink(storage)]
#[derive(Default, OwnableStorage)]
pub struct MyOwnable {
    #[OwnableStorageField]
    ownale: OwnableData,
}
```

## Step 4: Inherit logic

Inherit implementation of the `Ownable` trait. You can customize (override) methods in this `impl` block.

```rust
impl Ownable for MyOwnable {}
```

## Step 5: Define constructor

Define the constructor and initialize the owner with the contract initiator. Your basic version of `Ownable` contract is ready!

```rust
impl MyOwnable {
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

Customize it by adding ownable logic. We will add a `owner_function` to `MyOwnable` implemenation and add the `only_owner` modifier, which will verify that the caller of the function is the owner.

```rust
#![cfg_attr(not(feature = "std"), no_std)]

#[brush::contract]
pub mod ownable {
   use brush::{
      modifiers,
      traits::InkStorage,
   };
   use ink_prelude::vec::Vec;
   use ownable::traits::*;

   #[ink(storage)]
   #[derive(Default, OwnableStorage)]
   pub struct MyOwnable {
      #[OwnableStorageField]
      ownable: OwnableData,
   }

   impl Ownable for MyOwnable {}
    
   impl MyOwnable {
      
      #[ink(constructor)]
      pub fn new() -> Self {
         let mut instance = Self::default();
         let caller = instance.env().caller();
         instance._init_with_owner(caller);
         instance
      }

      #[ink(message)]
      #[modifiers(only_owner)]
      pub fn owner_function(&mut self) {
         // TODO
      }

   }

}

```

You can check the example of usage of [Ownable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/ownable).
