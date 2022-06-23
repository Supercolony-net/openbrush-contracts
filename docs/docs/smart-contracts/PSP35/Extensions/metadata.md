---
sidebar_position: 1
title: PSP35 Metadata
---

This example shows how you can reuse the implementation of [PSP35](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp35) token with [PSP35Metadata](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp35/extensions/metadata.rs) extension.

## Step 1: Add imports and enable unstable feature

Import **everything** from `openbrush::contracts::psp35::extensions::metadata`.

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

Declare storage struct and declare the field related to the `PSP35MetadataStorage` trait in addition to your `PSP35Storage` field. Then you need to derive the `PSP35MetadataStorage` trait and mark the corresponding field with the `#[PSP35MetadataStorageField]` attribute. Deriving this trait allows you to reuse the `PSP35Metadata` extension in your `PSP35` implementation.

```rust
#[derive(Default, SpreadAllocate, PSP35Storage, PSP35MetadataStorage)]
#[ink(storage)]
pub struct MyPSP35 {
    #[PSP35StorageField]
    psp35: PSP35Data,
    #[PSP35MetadataStorageField]
    metadata: PSP35MetadataData,
}
```

## Step 3: Inherit logic

Inherit implementation of the `PSP35Metadata` trait. You can customize (override) methods in this `impl` block.

```rust

impl PSP35 for MyPSP35 {}

impl PSP35Metadata for MyPSP35 {}
```

## Step 4: Define constructor

Define constructor. Your `PSP35Metadata` contract is ready!

```rust
impl MyPSP35 {
    #[ink(constructor)]
    pub fn new(id: Id, key: Vec<u8>, attribute: Vec<u8>) -> Self {
        ink_lang::codegen::initialize_contract(|instance: &mut Self| {
            instance.metadata.attributes.insert(&(id, key), &attribute);
        })
    }
}
```
You can check an example of the usage of [PSP35 Metadata](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp35_extensions/metadata).

You can also check the documentation for the basic implementation of [PSP35](/smart-contracts/PSP35).
