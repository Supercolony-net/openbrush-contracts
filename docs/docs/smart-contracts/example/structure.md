---
sidebar_position: 6
title: Structure and derive macro
---

The lending contract itself contains the data and the logic. OpenBrush allows us to easily separate the contract storage from the contract's logic. We will also be using our custom errors in the contract, for which we will use a separate file `errors.rs`. Our storage trait will need to be derived from the storage struct, for which we will use `brush_derive_macro`. For storage, we will create a file `traits.rs` and for the actual implementation of the contract, we will use the `lib.rs` file.

## Using derive macro

In our contract directory, we will create another directory called `derive`. This directory will contain the standard stuff of a Cargo folder - `.gitignore`, `Cargo.toml`, and the `lib.rs` file, inside of which we will derive our storage trait. So in the end, our `lib.rs` file will look like this:

```rust
#![cfg_attr(not(feature = "std"), no_std)]

extern crate proc_macro;

use brush_derive_macro::declare_derive_storage_trait;

declare_derive_storage_trait!(derive_lending_storage, LendingStorage, LendingStorageField);

```

This will of course work only after you added the correct dependencies:

```toml
[dependencies]
syn = { version = "1.0" }
quote = "1.0"
proc-macro2 = "1"
brush_derive_macro = { tag = "v1.2.0", git = "https://github.com/Supercolony-net/openbrush-contracts"}

[lib]
name = "lending_derive"
path = "lib.rs"
proc-macro = true

[features]
default = ["std"]
std = []
```