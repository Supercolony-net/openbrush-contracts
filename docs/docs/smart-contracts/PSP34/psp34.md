---
sidebar_position: 1
title: PSP34
---

This example shows how you can reuse the implementation of [PSP34](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp34) token. Also, this example shows how you can customize the logic, for example, to track the number of tokens minted with `next_id`, increasing it with each new token minted, securing a unique id for each token.

## Step 1: Include dependencies

Include `openbrush` as dependency in the cargo file or you can use [default `Cargo.toml`](/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush) template.
After you need to enable default implementation of PSP34 via `openbrush` features.

```toml
openbrush = { version = "~2.1.0", default-features = false, features = ["psp34"] }
```

## Step 2: Add imports and enable unstable feature

Use `openbrush::contract` macro instead of `ink::contract`. Import **everything** from `openbrush::contracts::psp34`.

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp34 {
    use openbrush::contracts::psp34::*;
    use ink_storage::traits::SpreadAllocate;
```

## Step 3: Define storage

Declare storage struct and declare the field related to the `PSP34Storage` trait. Then you need to derive the `PSP34Storage` trait and mark the corresponding field with the `#[PSP34StorageField]` attribute. Deriving this trait allows you to reuse the default implementation of `PSP34`.

```rust
#[ink(storage)]
#[derive(Default, SpreadAllocate, PSP34Storage)]
pub struct MyPSP34 {
    #[PSP34StorageField]
    psp34: PSP34Data,
}
```

## Step 4: Inherit logic

Inherit implementations of `PSP34` and `PSP34Metadata` traits. You can customize (override) methods in this `impl` block.

```rust
impl PSP34 for MyPSP34 {}

```

## Step 5: Define constructor

Define constructor. Your basic version of `PSP34` contract is ready!

```rust
impl MyPSP34 {
    #[ink(constructor)]
    pub fn new() -> Self {
        ink_lang::codegen::initialize_contract(|_instance: &mut Self| {})
    }
}
```

## Step 6: Customize your contract

Customize it by adding logic for tracking the number of minted tokens. It will contain a custom `mint_token` function which will handle the id of the newly minted token. Also, we will add the `next_id: u8` field to the structure, which will be increased with each newly minted token. This way we will make sure there will always be added a token with a unique id. 

```rust
#[ink(storage)]
#[derive(Default, SpreadAllocate, PSP34Storage)]
pub struct MyPSP34 {
    #[PSP34StorageField]
    psp34: PSP34Data,
    next_id: u8,
}

impl PSP34 for MyPSP34 {}

impl MyPSP34 {
    #[ink(constructor)]
    pub fn new() -> Self {
        ink_lang::codegen::initialize_contract(|_instance: &mut Self| {})
    }

    #[ink(message)]
    pub fn mint_token(&mut self) -> Result<(), PSP34Error> {
        self._mint_to(Self::env().caller(), Id::U8(self.next_id))?;
        self.next_id += 1;
        Ok(())
    }
}
```

You can check an example of the usage of [PSP34](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp34).
Also you can use extensions for psp34 token:

[PSP34Metadata](/smart-contracts/PSP34/extensions/metadata): metadata for PSP34.

[PSP34Mintable](/smart-contracts/PSP34/extensions/mintable): creation of new tokens.

[PSP34Burnable](/smart-contracts/PSP34/extensions/burnable): destruction of contract's tokens.

[PSP34Enumerable](/smart-contracts/PSP34/extensions/enumerable): iterating over contract's tokens.
