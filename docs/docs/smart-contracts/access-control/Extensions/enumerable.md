---
sidebar_position: 1
title: AccessControl Enumerable
---

This example shows how you can reuse the implementation of [AccessControl](https://github.com/Supercolony-net/openbrush-contracts/blob/main/contracts/src/access/access_control/access_control.rs) with [AccessControlEnumerable](https://github.com/Supercolony-net/openbrush-contracts/blob/main/contracts/src/access/access_control/extensions/enumerable.rs) extension, which enables an easier overview of access control roles.

First, you should implement basic version of [AccessControl](/smart-contracts/access-control).

## Step 1: Add imports and enable unstable feature

Import **everything** from `openbrush::contracts::access_control::extensions::enumerable`.

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_access_control {
    use openbrush::contracts::access_control::extensions::enumerable::*;
...
```

## Step 2: Define storage

Pass `enumerable::Members` into `access_control::Data` to be able to use `AcessControlEnumerable` extension in your `AccessControl` implementation.

```rust
#[derive(Default, Storage)]
#[ink(storage)]
pub struct Contract {
    #[storage_field]
    access: access_control::Data<enumerable::Members>,
}
```

## Step 3: Inherit logic

Inherit implementation of the `AccessControlEnumerable` trait. You can customize (override) methods in this `impl` block.

```rust

impl AccessControl for Contract {}

impl AccessControlEnumerable for Contract {}
```

## Final code

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_access_control {
    use openbrush::{
        contracts::access_control::extensions::enumerable::*,
        traits::Storage,
    };

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        access: access_control::Data<enumerable::Members>,
    }

    impl AccessControl for Contract {}
    impl AccessControlEnumerable for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
    }
}
```

And that's it! Your `AccessControl` is now extended by the `AccessControlEnumerable` extension and ready to use its functions!
You can check an example of the usage of [AccessControl Enumerable](https://github.com/Supercolony-net/openbrush-contracts/blob/main/contracts/src/access/access_control/extensions/enumerable.rs).

You can also check the documentation for the basic implementation of [AccessControl](/smart-contracts/access-control).