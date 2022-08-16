---
sidebar_position: 1
title: PSP1155 Metadata
---

This example shows how you can reuse the implementation of [PSP1155](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp1155) token with [PSP1155Metadata](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp1155/src/extensions/metadata.rs) extension.

## Step 1: Add imports and enable unstable feature

Import **everything** from `openbrush::contracts::psp1155::extensions::metadata`.

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp1155 {
    use openbrush::contracts::psp1155::extensions::metadata::*;
    use ink_prelude::string::String;
    use ink_storage::traits::SpreadAllocate;
...
```

## Step 2: Define storage

Declare storage struct and declare the field related to the `PSP1155MetadataStorage` trait in addition to your `PSP1155Storage` field. Then you need to derive the `PSP1155MetadataStorage` trait and mark the corresponding field with the `#[PSP1155MetadataStorageField]` attribute. Deriving this trait allows you to reuse the `PSP1155Metadata` extension in your `PSP1155` implementation.

```rust
#[derive(Default, SpreadAllocate, PSP1155Storage, PSP1155MetadataStorage)]
#[ink(storage)]
pub struct MyPSP1155 {
    #[PSP1155StorageField]
    psp1155: PSP1155Data,
    #[PSP1155MetadataStorageField]
    metadata: PSP1155MetadataData,
}
```

## Step 3: Inherit logic

Inherit implementation of the `PSP1155Metadata` trait. You can customize (override) methods in this `impl` block.

```rust

impl PSP1155 for MyPSP1155 {}

impl PSP1155Metadata for MyPSP1155 {}
```

## Step 4: Define constructor

Define constructor. Your `PSP1155Metadata` contract is ready!

```rust
impl MyPSP1155 {
    #[ink(constructor)]
    pub fn new(uri: Option<String>) -> Self {
        ink_lang::codegen::initialize_contract(|instance: &mut Self| {
            instance.metadata.uri = uri;
        })
    }
}
```
You can check an example of the usage of [PSP1155 Metadata](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp1155_extensions/metadata).

You can also check the documentation for the basic implementation of [PSP1155](../psp1155.md).
