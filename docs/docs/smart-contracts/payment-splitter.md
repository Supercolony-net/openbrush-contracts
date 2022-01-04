---
sidebar_position: 7
title: Payment Splitter
---

This example shows how you can reuse the implementation of
[payment-splitter](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/finance/payment_splitter).

## Step 1: Include dependencies

Include `brush` as dependency in the cargo file or you can use [default `Cargo.toml`](/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush) template.
After you need to enable a default implementation of Payment Splitter via features of the `brush`.

```toml
brush = { tag = "v1.2.0", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false, features = ["payment_splitter"] }

# payment-splitter uses dividing inside, so your version of rust can require you to disable check overflow.
[profile.dev]
overflow-checks = false

[profile.release]
overflow-checks = false
```

## Step 2: Add imports and enable unstable feature

Use `brush::contract` macro instead of `ink::contract`. Import **everything** from `brush::contracts::payment_splitter`.

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod my_payment_splitter {
    use brush::contracts::payment_splitter::*;
    use ink_prelude::vec::Vec;
```

## Step 3: Define storage

Declare storage struct and declare the field related to `PaymentSplitterStorage`
Then you need to derive `PaymentSplitterStorage` trait and mark corresponding field
with `#[PaymentSplitterStorageField]` attribute. Deriving this trait allows you to reuse
the default implementation of `PaymentSplitter`.

```rust
#[ink(storage)]
#[derive(Default, PaymentSplitterStorage)]
pub struct SplitterStruct {
   #[PaymentSplitterStorageField]
   splitter: PaymentSplitterData,
}
```

## Step 4: Inherit logic

Inherit the implementation of `PaymentSplitter`. You can customize (override) methods in this `impl` block.

```rust
impl PaymentSplitter for SplitterStruct {}
```

## Step 5: Define constructor

Define constructor. Your basic version of `PaymentSplitter` contract is ready!

```rust
impl SplitterStruct {
   #[ink(constructor)]
   pub fn new(payees: Vec<AccountId>, shares: Vec<Balance>) -> Self {
      let mut instance = Self::default();
      instance._init(payees, shares);
      instance
   }
}
```

You can check an example of the usage of [PaymentSplitter](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/payment_splitter).