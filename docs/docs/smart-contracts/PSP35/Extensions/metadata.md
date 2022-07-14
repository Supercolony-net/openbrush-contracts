---
sidebar_position: 1
title: PSP35 Metadata
---

This example shows how you can reuse the implementation of [PSP35](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp35) token with [PSP35Metadata](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp35/extensions/metadata.rs) extension.

First, you should implement basic version of [PSP35](/smart-contracts/PSP35).

## Step 1: Add imports and enable unstable feature

Import **everything** from `openbrush::contracts::psp35::extensions::metadata`.

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp35 {
    use openbrush::contracts::psp35::extensions::metadata::*;
...
```

## Step 2: Define storage

Declare storage struct and declare the field related to the metadata module. 
Then you need to derive the `Storage` trait and mark the corresponding field with 
the `#[storage_field]` attribute. 
Deriving this trait allows you to reuse the `PSP35Metadata` extension in your 
`PSP35` implementation.

```rust
#[derive(Default, SpreadAllocate, Storage)]
#[ink(storage)]
pub struct Contract {
    ...
    #[storage_field]
    metadata: metadata::Data,
}
```

## Step 3: Inherit logic

Inherit implementation of the `PSP35Metadata` trait. You can customize (override) methods in this `impl` block.

```rust
impl PSP35Metadata for Contract {}
```

## Step 4: Define constructor

Define constructor. Your `PSP35Metadata` contract is ready!

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
pub mod my_psp35 {
    use ink_prelude::vec::Vec;
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::psp35::extensions::metadata::*,
        traits::Storage,
    };

    #[derive(Default, SpreadAllocate, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        psp35: psp35::Data,
        #[storage_field]
        metadata: metadata::Data,
    }

    impl PSP35 for Contract {}

    impl PSP35Metadata for Contract {}

    impl Contract {
        /// contract constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self| {})
        }

        #[ink(message)]
        pub fn set_attribute(&mut self, id: Id, key: Vec<u8>, data: Vec<u8>) -> Result<(), PSP35Error> {
            self._set_attribute(&id, &key, &data)
        }
    }
}
```

You can check an example of the usage of [PSP35 Metadata](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp35_extensions/metadata).

You can also check the documentation for the basic implementation of [PSP35](/smart-contracts/PSP35).
