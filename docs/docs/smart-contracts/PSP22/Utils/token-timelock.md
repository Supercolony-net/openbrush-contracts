---
sidebar_position: 1
title: PSP22 Token Timelock
---

This example shows how you can reuse the implementation of [PSP22 Token Timelock](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp22/src/utils/token_timelock.rs) utility for [PSP22](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp22). This contract will lock user's `PSP22` tokens until the time specified, when they can withdraw them.

## Step 1: Include dependencies

Include `openbrush` as dependency in the cargo file or you can use [default `Cargo.toml`](/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush) template.
After you need to enable default implementation of PSP22 via `openbrush` features.

```toml
openbrush = { version = "~2.1.0", default-features = false, features = ["psp22"] }
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
...
```

## Step 3: Define storage

Declare storage struct and declare the field related to the `PSP22TokenTimelockStorage` trait. Then you need to derive the `PSP22TokenTimelockStorage` trait and mark the corresponding field with `#[PSP22TokenTimelockStorageField]` attribute. Deriving this trait allows you to reuse the default implementation of and `PSP22TokenTimelock`.

```rust
#[ink(storage)]
#[derive(Default, SpreadAllocate, PSP22TokenTimelockStorage)]
pub struct MyPSP22TokenTimelock {
    #[PSP22TokenTimelockStorageField]
    timelock: PSP22TokenTimelockData
}
```

## Step 4: Inherit logic

Inherit the implementation of the `PSP22TokenTimelock` trait. You can customize (override) methods in this `impl` block.

```rust
impl PSP22TokenTimelock for MyPSP22TokenTimelock {}
```

## Step 5: Define constructor

Define constructor. Your implementation of `PSP22TokenTimelock` contract is ready!

```rust
impl MyPSP22TokenTimelock {
    #[ink(constructor)]
    pub fn new(token_address: AccountId, beneficiary: AccountId, release_time: Timestamp) -> Self {
        ink_lang::codegen::initialize_contract(|instance: &mut Self| {
            assert!(instance._init(token_address, beneficiary, release_time).is_ok());
        })
    }
}
```

You can check an example of the usage of [PSP22 Token Timelock](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp22_utils/token_timelock).

You can also check the documentation for the basic implementation of [PSP22](/smart-contracts/PSP22).