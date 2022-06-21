---
sidebar_position: 8
title: Timelock Controller
---

This example shows how you can reuse the implementation of
[timelock-controller](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/governance/timelock_controller).

## Step 1: Include dependencies

Include `openbrush` as dependency in the cargo file or you can use [default `Cargo.toml`](/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush) template.
After you need to enable default implementation of Timelock Controller via `openbrush` features.

```toml
openbrush = { version = "~2.1.0", default-features = false, features = ["timelock_controller"] }
```

## Step 2: Add imports and enable unstable feature

Use `openbrush::contract` macro instead of `ink::contract`. Import **everything** from `openbrush::contracts::psp22::utils::token_timelock`.

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22_token_timelock {
    use openbrush::contracts::psp22::utils::token_timelock::*;
    use ink_storage::traits::SpreadAllocate;
```

## Step 3: Define storage

`TimelockController` is an extension for `AccessControl`, so you need to impl stuff related to both modules.
Declare storage struct and declare the fields related to `TimelockControllerStorage` and `AccessControlStorage`.
Then you need to derive `TimelockControllerStorage` and `AccessControlStorage` traits and mark corresponding fields
with `#[TimelockControllerStorageField]` and `#[AccessControlStorageField]` attributes. 
Deriving these traits allows you to reuse the default implementation of `TimelockController`(and `AccessControl`).

```rust
#[ink(storage)]
#[derive(Default, SpreadAllocate, TimelockControllerStorage)]
pub struct TimelockStruct {
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
      ink_lang::codegen::initialize_contract(|instance: &mut Self| {
            let caller = instance.env().caller();
            // `TimelockController` and `AccessControl` have `_init_with_admin` methods.
            // You need to call it for each trait separately, to initialize everything for these traits.
            AccessControlInternal::_init_with_admin(instance, caller);
            TimelockControllerInternal::_init_with_admin(instance, caller, min_delay, proposers, executors);
      })
   }
}
```

You can check an example of the usage of [TimelockController](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/timelock_controller).