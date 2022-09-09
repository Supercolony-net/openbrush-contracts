---
sidebar_position: 1
title: PSP34 Metadata
---

This example shows how you can reuse the implementation of [PSP34](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp34) token with [PSP34Metadata](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp34/extensions/metadata.rs) extension.

## Step 1: Include dependencies

Include `openbrush` as dependency in the cargo file or you can use [default `Cargo.toml`](/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush) template.
After you need to enable default implementation of PSP34 via `openbrush` features.

```toml
openbrush = { version = "~2.1.0", default-features = false, features = ["psp34"] }
```

## Step 2: Add imports and enable unstable feature

Use `openbrush::contract` macro instead of `ink::contract`. Import **everything** from `openbrush::contracts::psp34::extensions::metadata`.

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp34_metadata {
    use openbrush::contracts::psp34::extensions::metadata::*;
    use ink_prelude::string::String;
    use ink_storage::traits::SpreadAllocate;
...
```

## Step 3: Define storage

Declare storage struct and declare the field related to the `PSP34MetadataStorage` trait in addition to your `PSP34Storage` field. Then you need to derive the `PSP34MetadataStorage` trait and mark the corresponding field with the `#[PSP34MetadataStorageField]` attribute. Deriving this trait allows you to reuse the `PSP34Metadata` extension in your `PSP34` implementation.

```rust
#[ink(storage)]
#[derive(Default, SpreadAllocate, PSP34Storage, PSP34MetadataStorage)]
pub struct MyPSP34 {
    #[PSP34StorageField]
    psp34: PSP34Data,
    #[PSP34MetadataStorageField]
    metadata: PSP34MetadataData,
}
```

## Step 4: Inherit logic

Inherit implementation of the `PSP34Metadata` trait. You can customize (override) methods in this `impl` block.

```rust
impl PSP34 for MyPSP34 {}
impl PSP34Metadata for MyPSP34 {}
// Optionally you can add more default implementations
impl PSP34Internal for MyPSP34 {}
impl PSP34MetadataInternal for MYPSP34 {}
```

## Step 5: Define constructor

Define constructor. Your `PSP34Metadata` contract is ready!

```rust
impl MyPSP34 {
    #[ink(constructor)]
    pub fn new(id: Id, name: String, symbol: String) -> Self {
        ink_lang::codegen::initialize_contract(|instance: &mut Self| {
            instance._set_attribute(id.clone(), String::from("name").into_bytes(), name.into_bytes());
            instance._set_attribute(id, String::from("symbol").into_bytes(), symbol.into_bytes());
        }
    }
}
```

You can check an example of the usage of [PSP34 Metadata](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp34_extensions/metadata).

You can also check the documentation for the basic implementation of [PSP34](/smart-contracts/PSP34).
