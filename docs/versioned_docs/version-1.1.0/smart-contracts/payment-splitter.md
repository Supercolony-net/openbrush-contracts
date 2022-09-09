---
sidebar_position: 7
title: Payment Splitter
---

This example shows how you can reuse the implementation of
[payment-splitter](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/finance/payment-splitter).

## Step 1: Include dependencies

Include dependencies to `payment-splitter` and `brush` in the cargo file.

```toml
[dependencies]
ink_primitives = { tag = "v3.0.0-rc6", git = "https://github.com/paritytech/ink", default-features = false }
ink_metadata = { tag = "v3.0.0-rc6", git = "https://github.com/paritytech/ink", default-features = false, features = ["derive"], optional = true }
ink_env = { tag = "v3.0.0-rc6", git = "https://github.com/paritytech/ink", default-features = false }
ink_storage = { tag = "v3.0.0-rc6", git = "https://github.com/paritytech/ink", default-features = false }
ink_lang = { tag = "v3.0.0-rc6", git = "https://github.com/paritytech/ink", default-features = false }
ink_prelude = { tag = "v3.0.0-rc6", git = "https://github.com/paritytech/ink", default-features = false }

scale = { package = "parity-scale-codec", version = "2", default-features = false, features = ["derive"] }
scale-info = { version = "1", default-features = false, features = ["derive"], optional = true }

# These dependencies
payment-splitter = { tag = "v1.0.0", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }
brush = { tag = "v1.0.0", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }


# payment-splitter uses dividing inside, so your version of rust can require you to disable check overflow.
[profile.dev]
overflow-checks = false

[profile.release]
overflow-checks = false

[features]
default = ["std"]
std = [
   "ink_primitives/std",
   "ink_metadata",
   "ink_metadata/std",
   "ink_env/std",
   "ink_storage/std",
   "ink_lang/std",
   "scale/std",
   "scale-info",
   "scale-info/std",

   # These dependencies   
   "payment-splitter/std",
   "brush/std",
]
```

## Step 2: Add imports

Replace `ink::contract` macro by `brush::contract`. 
Import **everything** from `payment_splitter::traits`.

```rust
#[brush::contract]
pub mod my_payment_splitter {
   use payment_splitter::traits::*;
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