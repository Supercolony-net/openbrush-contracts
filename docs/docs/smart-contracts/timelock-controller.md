---
sidebar_position: 8
title: Timelock Controller
---

This example shows how you can reuse the implementation of
[timelock-controller](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/governance/timelock_controller).

## Step 1: Import default implementation

With [default `Cargo.toml`](/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush),
you need to import the `timelock_controller` and `access_controll` modules, enable corresponding features, and embed modules data structures
as described in [that section](/smart-contracts/overview#reuse-implementation-of-traits-from-openbrush).

The main traits are `AccessControl` and `TimelockController`.

## Step 2: Define constructor

Define constructor where you init admin of the contract.

```rust
impl Contract {
    #[ink(constructor)]
    pub fn new(min_delay: Timestamp, proposers: Vec<AccountId>, executors: Vec<AccountId>) -> Self {
        ink_lang::codegen::initialize_contract(|instance: &mut Self| {
            let caller = instance.env().caller();
            // `TimelockController` and `AccessControl` have `_init_with_admin` methods.
            // You need to call it for each trait separately, to initialize everything for these traits.
            access_control::Internal::_init_with_admin(instance, caller);
            timelock_controller::Internal::_init_with_admin(instance, caller, min_delay, proposers, executors);
        })
    }
}
```

## Final code

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_timelock_controller {
    use ink_prelude::vec::Vec;
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::timelock_controller::*,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct Contract {
        #[storage_field]
        access_control: access_control::Data,
        #[storage_field]
        timelock: timelock_controller::Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(min_delay: Timestamp, proposers: Vec<AccountId>, executors: Vec<AccountId>) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                let caller = instance.env().caller();
                // `TimelockController` and `AccessControl` have `_init_with_admin` methods.
                // You need to call it for each trait separately, to initialize everything for these traits.
                access_control::Internal::_init_with_admin(instance, caller);
                timelock_controller::Internal::_init_with_admin(instance, caller, min_delay, proposers, executors);
            })
        }
    }

    // `TimelockController` is an extension for `AccessControl`, so you have to inherit logic related to both modules.
    impl AccessControl for Contract {}
    impl TimelockController for Contract {}
}
```

You can check an example of the usage of [TimelockController](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/timelock_controller).