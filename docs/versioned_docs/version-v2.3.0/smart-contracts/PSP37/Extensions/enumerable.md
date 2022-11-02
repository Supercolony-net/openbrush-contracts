---
sidebar_position: 1
title: PSP37 Enumerable
---

This example shows how you can reuse the implementation of [PSP37](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp37) token with [PSP37Enumerable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp37/extensions/enumerable.rs) extension.

First, you should implement basic version of [PSP37](/smart-contracts/PSP37).

## Step 1: Add imports and enable unstable feature

Import **everything** from `openbrush::contracts::psp37::extensions::enumerable`.

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp37 {
    use openbrush::contracts::psp37::extensions::enumerable::*;
...
```

## Step 2: Define storage

Pass `enumerable::Balances` into `psp37::Data` to be able to use `PSP37Enumerable` extension in your `PSP37` implementation.

```rust
#[derive(Default, SpreadAllocate, Storage)]
#[ink(storage)]
pub struct Contract {
    #[storage_field]
    psp37: psp37::Data<enumerable::Balances>,
}
```

## Step 3: Inherit logic

Inherit implementation of the `PSP37Enumerable` trait. You can customize (override) methods in this `impl` block.

```rust

impl PSP37 for Contract {}

impl PSP37Enumerable for Contract {}
```

## Final code

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp37_enumerable {
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::psp37::extensions::enumerable::*,
        traits::Storage,
    };

    #[derive(Default, SpreadAllocate, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        psp37: psp37::Data<enumerable::Balances>,
    }

    impl PSP37 for Contract {}
    impl PSP37Enumerable for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self| {})
        }
    }
}
```

And that's it! Your `PSP37` is now extended by the `PSP37Enumerable` extension and ready to use its functions!
You can check an example of the usage of [PSP37 Enumerable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp37_extensions/enumerable).

You can also check the documentation for the basic implementation of [PSP37](/smart-contracts/PSP37).