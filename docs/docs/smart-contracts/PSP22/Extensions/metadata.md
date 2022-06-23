---
sidebar_position: 1
title: PSP22 Metadata
---

This example shows how you can reuse the implementation of [PSP22](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp22) token with the [PSP22Metadata](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp22/extensions/metadata.rs) extension.

## Step 1: Add imports and enable unstable feature

Use `openbrush::contract` macro instead of `ink::contract`. Import **everything** from `openbrush::contracts::psp22::extensions::metadata`.

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22 {
    use openbrush::contracts::psp22::extensions::metadata::*;
    use ink_prelude::string::String;
    use ink_storage::traits::SpreadAllocate;
```

## Step 2: Define storage

Declare the storage struct and declare the field related to the `PSP22MetadataStorage` trait in addition to your `PSP22Storage` field. Then you need to derive the `PSP22MetadataStorage` trait and mark the corresponding field with the `#[PSP22MetadataStorageField]` attribute. Deriving this trait allows you to reuse the `PSP22Metadata` extension in your `PSP22` implementation.

```rust
#[ink(storage)]
#[derive(Default, SpreadAllocate, PSP22Storage, PSP22MetadataStorage)]
pub struct MyPSP22 {
    #[PSP22StorageField]
    psp22: PSP22Data,
    #[PSP22MetadataStorageField]
    metadata: PSP22MetadataData,
}
```

## Step 3: Inherit logic

Inherit the implementation of the `PSP22Metadata` trait. You can customize (override) methods in this `impl` block.

Inherit the implementation of the `PSP22` trait.

```rust
impl PSP22 for MyPSP22 {}

impl PSP22Metadata for MyPSP22 {}
```

## Step 4: Define constructor

Define constructor. Your `PSP22Metadata` contract is ready!

```rust
impl MyPSP22 {
    #[ink(constructor)]
    pub fn new(total_supply: Balance, name: Option<String>, symbol: Option<String>, decimal: u8) -> Self {
        ink_lang::codegen::initialize_contract(|instance: &mut Self| {
            instance.metadata.name = name;
            instance.metadata.symbol = symbol;
            instance.metadata.decimals = decimal;
            instance
                ._mint(instance.env().caller(), total_supply)
                .expect("Should mint total_supply");
        })
    }
}
```

You can check an example of the usage of [PSP22 Metadata](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp22_extensions/metadata).

You can also check the documentation for the basic implementation of [PSP22](/smart-contracts/PSP22).
