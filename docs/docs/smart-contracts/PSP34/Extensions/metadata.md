---
sidebar_position: 1
title: PSP34 Metadata
---

This example shows how you can reuse the implementation of [PSP34](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp34) token with [PSP34Metadata](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp34/extensions/metadata.rs) extension.

First, you should implement basic version of [PSP34](/smart-contracts/PSP34).

## Step 1: Add imports and enable unstable feature

Use `openbrush::contract` macro instead of `ink::contract`. Import **everything** from 
`openbrush::contracts::psp34::extensions::metadata`.

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp34_metadata {
    use openbrush::contracts::psp34::extensions::metadata::*;
...
```

## Step 2: Define storage

Declare storage struct and declare the field related to the metadata module data structure. 
Then you need to derive the `Storage` trait and mark the corresponding field with 
the `#[storage_field]` attribute. Deriving this trait allows you to reuse the 
`PSP34Metadata` extension in your `PSP34` implementation.

```rust
#[ink(storage)]
#[derive(Default, SpreadAllocate, Storage)]
pub struct Contract {
    ...
    #[storage_field]
    metadata: metadata::Data,
}
```

## Step 3: Inherit logic

Inherit implementation of the `PSP34Metadata` trait. You can customize (override) methods in this `impl` block.

```rust
impl PSP34Metadata for Contract {}
```

## Step 4: Define constructor

Define constructor. Your `PSP34Metadata` contract is ready!

```rust
impl Contract {
    #[ink(constructor)]
    pub fn new(id: Id, name: String, symbol: String) -> Self {
        ink_lang::codegen::initialize_contract(|instance: &mut Self| {
            instance._set_attribute(id.clone(), String::from("name").into_bytes(), name.into_bytes());
            instance._set_attribute(id, String::from("symbol").into_bytes(), symbol.into_bytes());
        }
    }
}
```

## Final code

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp34_metadata {
    use ink_prelude::{
        string::String,
        vec::Vec,
    };
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::psp34::extensions::metadata::*,
        traits::Storage,
    };

    #[derive(Default, SpreadAllocate, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        metadata: metadata::Data,
    }

    impl PSP34 for Contract {}

    impl PSP34Metadata for Contract {}

    impl Contract {
        /// A constructor which mints the first token to the owner
        #[ink(constructor)]
        pub fn new(id: Id, name: String, symbol: String) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                let name_key: Vec<u8> = String::from("name").into_bytes();
                let symbol_key: Vec<u8> = String::from("symbol").into_bytes();
                instance._set_attribute(id.clone(), name_key, name.into_bytes());
                instance._set_attribute(id, symbol_key, symbol.into_bytes());
            })
        }
    }
}
```

You can check an example of the usage of [PSP34 Metadata](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp34_extensions/metadata).

You can also check the documentation for the basic implementation of [PSP34](/smart-contracts/PSP34).
