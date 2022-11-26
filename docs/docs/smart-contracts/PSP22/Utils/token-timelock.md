---
sidebar_position: 1
title: PSP22 Token Timelock
---

This example shows how you can reuse the implementation of [PSP22 Token Timelock](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp22/src/utils/token_timelock.rs) utility for [PSP22](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp22). This contract will lock user's `PSP22` tokens until the time specified, when they can withdraw them.

## Step 1: Add imports and enable unstable feature

Use `openbrush::contract` macro instead of `ink::contract`. 
Import **everything** from `openbrush::contracts::psp22::utils::token_timelock`.

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22_token_timelock {
    use openbrush::contracts::psp22::utils::token_timelock::*;
...
```

## Step 2: Define storage

Declare storage struct and declare the field related to the timelock module data structure.
Then you need to derive the `Storage` trait and mark the corresponding field with
the `#[storage_field]` attribute. Deriving this trait allows you to reuse the
`PSP22TokenTimelock`.

```rust
#[ink(storage)]
#[derive(Default, Storage)]
pub struct Contract {
    #[storage_field]
    timelock: token_timelock::Data,
}
```

## Step 3: Inherit logic

Inherit the implementation of the `PSP22TokenTimelock` trait. 
You can customize (override) methods in this `impl` block.

```rust
impl PSP22TokenTimelock for Contract {}
```

## Step 4: Define constructor

Define constructor. Your implementation of `PSP22TokenTimelock` contract is ready!

```rust
impl Contract {
    #[ink(constructor)]
    pub fn new(token_address: AccountId, beneficiary: AccountId, release_time: Timestamp) -> Self {
        ink::codegen::initialize_contract(|instance: &mut Self| {
            assert!(instance._init(token_address, beneficiary, release_time).is_ok());
        })
    }
}
```

## Final code
```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22_token_timelock {
    use openbrush::{
        contracts::psp22::utils::token_timelock::*,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        timelock: token_timelock::Data,
    }

    impl PSP22TokenTimelock for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new(token_address: AccountId, beneficiary: AccountId, release_time: Timestamp) -> Self {
            ink::codegen::initialize_contract(|instance: &mut Self| {
                assert!(instance._init(token_address, beneficiary, release_time).is_ok());
            })
        }
    }
}
```

You can check an example of the usage of [PSP22 Token Timelock](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp22_utils/token_timelock).

You can also check the documentation for the basic implementation of [PSP22](/smart-contracts/PSP22).