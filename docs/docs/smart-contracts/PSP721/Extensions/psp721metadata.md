---
sidebar_position: 1
title: PSP721 Metadata
---

This example shows how you can reuse the implementation of [PSP721](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp721) token with [PSP721Metadata](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp721/src/extensions/metadata.rs) extension.

## Step 1: Add imports

Import **everything** from `psp721::extensions::metadata`.

```rust
#[brush::contract]
pub mod my_psp721 {
   use psp721::extensions::metadata::*;
```

## Step 2: Define storage

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

## Step 3: Inherit logic

Inherit implementation of the `PSP721Metadata` trait. You can customize (override) methods in this `impl` block.

```rust
impl PSP721Metadata for MyPSP721 {}
```

## Step 4: Define constructor

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
