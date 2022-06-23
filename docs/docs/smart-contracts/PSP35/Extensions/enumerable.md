---
sidebar_position: 1
title: PSP35 Enumerable
---

This example shows how you can reuse the implementation of [PSP35](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp35) token with [PSP35Enumerable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp35/extensions/enumerable.rs) extension.

## Step 1: Add imports and enable unstable feature

Import **everything** from `openbrush::contracts::psp35::extensions::enumerable`.

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp35 {
    use openbrush::contracts::psp35::extensions::metadata::*;
    use ink_prelude::string::String;
    use ink_storage::traits::SpreadAllocate;
...
```

## Step 2: Define storage

Declare storage struct and use `EnumerableBalances` instead of common balances to be able to use `PSP35Enumerable` extension in your `PSP35` implementation.

```rust
#[derive(Default, SpreadAllocate, PSP35Storage)]
#[ink(storage)]
pub struct MyPSP35 {
    #[PSP35StorageField]
    psp35: PSP35Data<EnumerableBalances>,
}
```

## Step 3: Inherit logic

Inherit implementation of the `PSP35Enumerable` trait. You can customize (override) methods in this `impl` block.

```rust

impl PSP35 for MyPSP35 {}

impl PSP35Enumerable for MyPSP35 {}
```

And that's it! Your `PSP35` is now extended by the `PSP35Enumerable` extension and ready to use its functions!
You can check an example of the usage of [PSP35 Enumerable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp35_extensions/enumerable).

You can also check the documentation for the basic implementation of [PSP35](/smart-contracts/PSP35).