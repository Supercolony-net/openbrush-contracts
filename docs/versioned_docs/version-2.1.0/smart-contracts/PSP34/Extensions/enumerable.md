---
sidebar_position: 3
title: PSP34 Enumerable
---

This example shows how you can reuse the implementation of [PSP34](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp34) token with [PSP34Enumerable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp34/extensions/enumerable.rs) extension.

## Step 1: Add imports and enable unstable feature

Import **everything** from `openbrush::contracts::psp34::extensions::enumerable`.

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp34 {
    use openbrush::contracts::psp34::extensions::enumerable::*;
    use ink_storage::traits::SpreadAllocate;
...
```

## Step 2: Define storage

Declare storage struct and use `EnumerableBalances` instead of common balances to be able to use `PSP34Enumerable` extension in your `PSP34` implementation.

```rust
#[derive(Default, SpreadAllocate, PSP34Storage)]
#[ink(storage)]
pub struct MyPSP34 {
    #[PSP34StorageField]
    psp34: PSP34Data<EnumerableBalances>,
}
```

## Step 3: Inherit logic

Inherit implementation of the `PSP34Enumerable` trait. You can customize (override) methods in this `impl` block.

```rust

impl PSP34 for MyPSP34 {}

impl PSP34Enumerable for MyPSP34 {}
```

And that's it! Your `PSP34` is now extended by the `PSP34Enumerable` extension and ready to use its functions!
You can check an example of the usage of [PSP34 Enumerable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp34_extensions/enumerable).

You can also check the documentation for the basic implementation of [PSP34](/smart-contracts/PSP34).