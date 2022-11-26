---
sidebar_position: 1
title: PSP37 Metadata
---

This example shows how you can reuse the implementation of [PSP37](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp37) token with [PSP37Metadata](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp37/extensions/metadata.rs) extension.

First, you should implement basic version of [PSP37](/smart-contracts/PSP37).

## Step 1: Add imports and enable unstable feature

Import **everything** from `openbrush::contracts::psp37::extensions::metadata`.

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp37 {
    use openbrush::contracts::psp37::extensions::metadata::*;
...
```

## Step 2: Define storage

Declare storage struct and declare the field related to the metadata module. 
Then you need to derive the `Storage` trait and mark the corresponding field with 
the `#[storage_field]` attribute. 
Deriving this trait allows you to reuse the `PSP37Metadata` extension in your 
`PSP37` implementation.

```rust
#[derive(Default, Storage)]
#[ink(storage)]
pub struct Contract {
    ...
    #[storage_field]
    metadata: metadata::Data,
}
```

## Step 3: Inherit logic

Inherit implementation of the `PSP37Metadata` trait. You can customize (override) methods in this `impl` block.

```rust
impl PSP37Metadata for Contract {}
```

## Step 4: Define constructor

Define constructor. Your `PSP37Metadata` contract is ready!

```rust
impl Contract {
    #[ink(constructor)]
    pub fn new(id: Id, key: Vec<u8>, attribute: Vec<u8>) -> Self {
        ink_lang::codegen::initialize_contract(|instance: &mut Self| {
            instance._set_attribute(&id, &key, &data)
        })
    }
}
```

## Final code

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp37 {
    use ink::prelude::vec::Vec;
    use openbrush::{
        contracts::psp37::extensions::metadata::*,
        traits::Storage,
    };

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        psp37: psp37::Data,
        #[storage_field]
        metadata: metadata::Data,
    }

    impl PSP37 for Contract {}

    impl PSP37Metadata for Contract {}

    impl Contract {
        /// contract constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self| {})
        }

        #[ink(message)]
        pub fn set_attribute(&mut self, id: Id, key: Vec<u8>, data: Vec<u8>) -> Result<(), PSP37Error> {
            self._set_attribute(&id, &key, &data)
        }
    }
}
```

You can check an example of the usage of [PSP37 Metadata](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp37_extensions/metadata).

You can also check the documentation for the basic implementation of [PSP37](/smart-contracts/PSP37).
