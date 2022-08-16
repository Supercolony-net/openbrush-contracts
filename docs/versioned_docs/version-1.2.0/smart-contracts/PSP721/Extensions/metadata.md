---
sidebar_position: 1
title: PSP721 Metadata
---

This example shows how you can reuse the implementation of [PSP721](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp721) token with [PSP721Metadata](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp721/src/extensions/metadata.rs) extension.

## Step 1: Include dependencies

Include `brush` as dependency in the cargo file or you can use [default `Cargo.toml`](/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush) template.
After you need to enable default implementation of PSP721 via `brush` features.

```toml
brush = { tag = "v1.2.0", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false, features = ["psp721"] }
```

## Step 2: Add imports and enable unstable feature

Use `brush::contract` macro instead of `ink::contract`. Import **everything** from `brush::contracts::psp721::extensions::metadata`.

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod my_psp721_metadata {
    use brush::contracts::psp721::extensions::metadata::*;
    use ink_prelude::string::String;
...
```

## Step 3: Define storage

Declare storage struct and declare the field related to the `PSP721MetadataStorage` trait in addition to your `PSP721Storage` field. Then you need to derive the `PSP721MetadataStorage` trait and mark the corresponding field with the `#[PSP721MetadataStorageField]` attribute. Deriving this trait allows you to reuse the `PSP721Metadata` extension in your `PSP721` implementation.

```rust
#[ink(storage)]
#[derive(Default, PSP721Storage, PSP721MetadataStorage)]
pub struct MyPSP721 {
    #[PSP721StorageField]
    psp721: PSP721Data,
    #[PSP721MetadataStorageField]
    metadata: PSP721MetadataData,
}
```

## Step 4: Inherit logic

Inherit implementation of the `PSP721Metadata` trait. You can customize (override) methods in this `impl` block.

```rust
impl PSP721Metadata for MyPSP721 {}
```

## Step 5: Define constructor

Define constructor. Your `PSP721Metadata` contract is ready!

```rust
impl MyPSP721 {
    #[ink(constructor)]
    pub fn new(name: Option<String>, symbol: Option<String>) -> Self {
        let mut instance = Self::default();
        instance.metadata.name = name;
        instance.metadata.symbol = symbol;
        instance
    }
}
```

You can check an example of the usage of [PSP721 Metadata](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp721_extensions/metadata).

You can also check the documentation for the basic implementation of [PSP721](../psp721.md).
