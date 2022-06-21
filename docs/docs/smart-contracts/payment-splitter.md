---
sidebar_position: 7
title: Payment Splitter
---

This example shows how you can reuse the implementation of
[payment-splitter](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/finance/payment_splitter).

## Step 1: Include dependencies

Include `openbrush` as dependency in the cargo file or you can use [default `Cargo.toml`](/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush) template.
After you need to enable default implementation of Payment Splitter via `openbrush` features.

```toml
openbrush = { version = "~2.1.0", default-features = false, features = ["payment_splitter"] }

# payment-splitter uses dividing inside, so your version of rust can require you to disable check overflow.
[profile.dev]
```

## Step 2: Add imports and enable unstable feature

Use `openbrush::contract` macro instead of `ink::contract`. Import **everything** from `openbrush::contracts::payment_splitter`.

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_payment_splitter {
    use openbrush::contracts::payment_splitter::*;
    use ink_prelude::vec::Vec;
    use ink_storage::traits::SpreadAllocate;
```

## Step 3: Define storage

Declare storage struct and declare the field related to `PaymentSplitterStorage`
Then you need to derive `PaymentSplitterStorage` trait and mark corresponding field
with `#[PaymentSplitterStorageField]` attribute. Deriving this trait allows you to reuse
the default implementation of `PaymentSplitter`.

```rust
#[ink(storage)]
#[derive(Default, SpreadAllocate, PaymentSplitterStorage)]
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
   pub fn new(payees_and_shares: Vec<(AccountId, Balance)>) -> Self {
      ink_lang::codegen::initialize_contract(|instance: &mut Self| {
         instance._init(payees_and_shares).expect("Should init");
      })
   }
}
```

You can check an example of the usage of [PaymentSplitter](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/payment_splitter).

## Step 6 (Optional): Customize your contract

The `PaymentSplitter` trait defines and has default implementations for the core payment splitter functions. Additional functionality with *some* predefined functions is available through the `PaymentSplitterInternal` trait (`openbrush-contracts/contracts/finance/payment_splitter/mod.rs`). Likely the most common function to use from this internal trait will be `_release_all`. This allows you to payout all `payees` stored in the contract at once. To add this function to your contract, simply define a new publicly dispatchable function (i.e. `#[ink(message)]`) called `release_all` and have it call the internal `_release_all` function using `self`.

```rust
impl SplitterStruct {
        #[ink(constructor)]
        pub fn new(payees_and_shares: Vec<(AccountId, Balance)>) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance._init(payees_and_shares).expect("Should init");
            })
        }

        /// Payout all payees at once.
        #[ink(message)]
        pub fn release_all(&mut self) -> Result<(), PaymentSplitterError> {
            // `_release_all()` is an internal method defined by the `PaymentSplitterInternal` trait
            self._release_all()
        }
    }
```
The `_add_payee` function is also available in the `PaymentSplitterInternal` trait and can be added to your contract in the same way as `_release_all`.