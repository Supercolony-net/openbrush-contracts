---
sidebar_position: 7
title: Timelock Controller
---

This example shows how you can reuse the implementation of
[timelock-controller](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/governance/timelock-controller).

## Step 1: Include dependencies

Include dependencies to `timelock-controller` and `brush` in the cargo file.

```toml
[dependencies]
ink_primitives = { tag = "v3.0.0-rc6", git = "https://github.com/paritytech/ink", default-features = false }
ink_metadata = { tag = "v3.0.0-rc6", git = "https://github.com/paritytech/ink", default-features = false, features = ["derive"], optional = true }
ink_env = { tag = "v3.0.0-rc6", git = "https://github.com/paritytech/ink", default-features = false }
ink_storage = { tag = "v3.0.0-rc6", git = "https://github.com/paritytech/ink", default-features = false }
ink_lang = { tag = "v3.0.0-rc6", git = "https://github.com/paritytech/ink", default-features = false }
ink_prelude = { tag = "v3.0.0-rc6", git = "https://github.com/paritytech/ink", default-features = false }

scale = { package = "parity-scale-codec", version = "2", default-features = false, features = ["derive"] }
scale-info = { version = "1", default-features = false, features = ["derive"], optional = true }

# These dependencies
timelock-controller = { tag = "v1.0.0", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }
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
   "timelock-controller/std",
   "brush/std",
]
```

## Step 2: Add imports

Replace `ink::contract` macro by `brush::contract`.
Import **everything** from `timelock_controller::traits`.

```rust
#[brush::contract]
pub mod my_timelock_controller {
   use timelock_controller::traits::*;
   use ink_prelude::vec::Vec;
```

## Step 3: Define storage

`TimelockController` is an extension for `AccessControl`, so you need to impl stuff related to both modules.
Declare storage struct and declare the fields related to `TimelockControllerStorage` and `AccessControlStorage`.
Then you need to derive `TimelockControllerStorage` and `AccessControlStorage` traits and mark corresponding fields
with `#[TimelockControllerStorageField]` and `#[AccessControlStorageField]` attributes. 
Deriving these traits allows you to reuse the default implementation of `TimelockController`(and `AccessControl`).

```rust
#[ink(storage)]
#[derive(Default, AccessControlStorage, TimelockControllerStorage)]
pub struct TimelockStruct {
   #[AccessControlStorageField]
   access: AccessControlData,
   #[TimelockControllerStorageField]
   timelock: TimelockControllerData,
}
```

## Step 4: Inherit logic

Inherit implementations of `TimelockController` and `AccessControl` traits. You can customize (override) methods in this `impl` block.

```rust
// `TimelockController` is an extension for `AccessControl`, so you need to impl stuff related to both modules.
impl AccessControl for TimelockStruct {}
impl TimelockController for TimelockStruct {}
```

## Step 5: Define constructor

Define constructor. Your basic version of `TimelockController` contract is ready!

```rust
impl TimelockStruct {
   #[ink(constructor)]
   pub fn new(min_delay: Timestamp, proposers: Vec<AccountId>, executors: Vec<AccountId>) -> Self {
      let mut instance = Self::default();
      let caller = instance.env().caller();
      // `TimelockController` and `AccessControl` have `_init_with_admin` methods.
      // You need to call it for each trait separately, to initialize everything for these traits.
      AccessControl::_init_with_admin(&mut instance, caller);
      TimelockController::_init_with_admin(&mut instance, caller, min_delay, proposers, executors);
      instance
   }
}
```
