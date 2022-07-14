---
sidebar_position: 1
title: PSP35 Enumerable
---

This example shows how you can reuse the implementation of [PSP35](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp35) token with [PSP35Enumerable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp35/extensions/enumerable.rs) extension.

First, you should implement basic version of [PSP35](/smart-contracts/PSP35).

## Step 1: Add imports and enable unstable feature

Import **everything** from `openbrush::contracts::psp35::extensions::enumerable`.

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp35 {
    use openbrush::contracts::psp35::extensions::enumerable::*;
...
```

## Step 2: Define storage

Pass `enumerable::Balances` into `psp35::Data` to be able to use `PSP35Enumerable` extension in your `PSP35` implementation.

```rust
#[derive(Default, SpreadAllocate, Storage)]
#[ink(storage)]
pub struct Contract {
    #[storage_field]
    psp35: psp35::Data<enumerable::Balances>,
}
```

## Step 3: Inherit logic

Inherit implementation of the `PSP35Enumerable` trait. You can customize (override) methods in this `impl` block.

```rust

impl PSP35 for Contract {}

impl PSP35Enumerable for Contract {}
```

## Final code

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp35_enumerable {
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::psp35::extensions::enumerable::*,
        traits::Storage,
    };

    #[derive(Default, SpreadAllocate, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        psp35: psp35::Data<enumerable::Balances>,
    }

    impl PSP35 for Contract {}
    impl PSP35Enumerable for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self| {})
        }
    }
}
```

And that's it! Your `PSP35` is now extended by the `PSP35Enumerable` extension and ready to use its functions!
You can check an example of the usage of [PSP35 Enumerable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp35_extensions/enumerable).

You can also check the documentation for the basic implementation of [PSP35](/smart-contracts/PSP35).