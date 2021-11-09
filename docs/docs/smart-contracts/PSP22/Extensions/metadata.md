---
sidebar_position: 1
title: PSP22 Metadata
---

This example shows how you can reuse the implementation of [PSP22](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp22) token with the [PSP22Metadata](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp22/extensions/Metadata.rs) extension.

## Step 1: Add imports

Replace `ink::contract` macro by `brush::contract` and import **everything** from `psp22::extensions::metadata`.

```rust
#[brush::contract]
pub mod my_psp22_metadata {
   use psp22::extensions::metadata::*;
```

## Step 2: Define storage

Declare the storage struct and declare the field related to the `PSP22MetadataStorage` trait in addition to your `PSP22Storage` field. Then you need to derive the `PSP22MetadataStorage` trait and mark the corresponding field with the `#[PSP22MetadataStorageField]` attribute. Deriving this trait allows you to reuse the `PSP22Metadata` extension in your `PSP22` implementation.

```rust
#[ink(storage)]
#[derive(Default, PSP22Storage, PSP22MetadataStorage)]
pub struct MyPSP22 {
    #[PSP22StorageField]
    psp22: PSP22Data,
    #[PSP22MetadataStorageField]
    metadata: PSP22MetadataData,
}
```

## Step 3: Inherit logic

Inherit the implementation of the `PSP22Metadata` trait. You can customize (override) methods in this `impl` block.

```rust
impl PSP22Metadata for MyPSP22 {}
```

## Step 4: Define constructor

Define constructor. Your `PSP22Metadata` contract is ready!

```rust
impl MyPSP22 {
   #[ink(constructor)]
   pub fn new(total_supply: Balance, name: Option<String>, symbol: Option<String>, decimal: u8) -> Self {
      let mut instance = Self::default();
      Lazy::set(&mut instance.metadata.name, name);
      Lazy::set(&mut instance.metadata.symbol,symbol);
      Lazy::set(&mut instance.metadata.decimals,decimal);
      instance._mint(instance.env().caller(), total_supply);
      instance
   }
}
```

You can also check the documentation for the basic implementation of [PSP22](/smart-contracts/PSP22/psp22).
