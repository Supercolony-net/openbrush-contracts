---
sidebar_position: 1
title: PSP35
---

This example shows how you can reuse the implementation of [PSP35](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp35) token. Also, this example shows how you can customize the logic, for example, to track the number of token types with `unique_ids`, adding a new token type with the `add_type` function.

## Step 1: Include dependencies

Include `openbrush` as dependency in the cargo file or you can use [default `Cargo.toml`](/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush) template.
After you need to enable default implementation of PSP35 via `openbrush` feature.

```toml
openbrush = { version = "~2.1.0", default-features = false, features = ["psp35"] }
```

## Step 2: Add imports and enable unstable feature

Use `openbrush::contract` macro instead of `ink::contract`. Import **everything** from `openbrush::contracts::psp35`.

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp35 {
    use openbrush::contracts::psp35::*;
    use ink_prelude::{
        string::String,
        vec,
    };
    use ink_storage::{
        traits::SpreadAllocate,
        Mapping,
    };
...
```

## Step 3: Define storage

Declare storage struct and declare the field related to the `PSP35Storage` trait. Then you need to derive the `PSP35Storage` trait and mark the corresponding field with the `#[PSP35StorageField]` attribute. Deriving this trait allows you to reuse the default implementation of `PSP35`.

```rust
#[derive(Default, SpreadAllocate, PSP35Storage)]
#[ink(storage)]
pub struct MyPSP35 {
    #[PSP35StorageField]
    psp35: PSP35Data,
}
```

## Step 4: Inherit logic

Inherit implementations of the `PSP35` trait. You can customize (override) methods in this `impl` block.

```rust
impl PSP35 for MyPSP35 {}
```

## Step 5: Define constructor

Define constructor. Your basic version of the `PSP35` contract is ready!

```rust
impl MyPSP35 {
    #[ink(constructor)]
    pub fn new() -> Self {
        ink_lang::codegen::initialize_contract(|_instance: &mut Self| {})
    }
}
```

## Step 6: Customize your contract

Customize it by adding logic for denying of minting some tokens. 
We can deny minting of token with id by `deny` function.
Id will be added to `denied_ids` map.
If someone tries to mint token with denied id, we will reject transaction.

```rust
#[derive(Default, SpreadAllocate, PSP35Storage)]
#[ink(storage)]
pub struct MyPSP35 {
    #[PSP35StorageField]
    psp35: PSP35Data,
    denied_ids: Mapping<Id, ()>,
}

impl PSP35 for MyPSP35 {}

impl MyPSP35 {
    #[ink(constructor)]
    pub fn new() -> Self {
        ink_lang::codegen::initialize_contract(|_instance: &mut Self| {})
    }

    #[ink(message)]
    pub fn deny(&mut self, id: Id) {
        self.denied_ids.insert(&id, &());
    }

    #[ink(message)]
    pub fn mint_tokens(&mut self, id: Id, amount: Balance) -> Result<(), PSP35Error> {
        if self.denied_ids.get(&id).is_some() {
            return Err(PSP35Error::Custom(String::from("Id is denied")))
        }
        self._mint_to(Self::env().caller(), vec![(id, amount)])
    }
}
```
You can check an example of the usage of [PSP35](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp35).

Also you can use extensions for PSP35 token:

[PSP35Metadata](/smart-contracts/PSP35/extensions/metadata): metadata for PSP35.

[PSP35Mintable](/smart-contracts/PSP35/extensions/mintable): creation of new tokens.

[PSP35Burnable](/smart-contracts/PSP35/extensions/burnable): destruction of contract's tokens.

[PSP35Batch](/smart-contracts/PSP35/extensions/batch): transfer batch of tokens.

[PSP35Enumerable](/smart-contracts/PSP35/extensions/enumerable): iterates over contract's tokens.