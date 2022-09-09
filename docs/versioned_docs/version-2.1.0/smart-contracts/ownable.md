---
sidebar_position: 3
title: Ownable
---

This example shows how you can use the implementation of [ownable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/access/ownable) to provide `only owner` rights for contract's functions.

## Step 1: Include dependencies

Include `openbrush` as dependency in the cargo file or you can use [default `Cargo.toml`](/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush) template.
After you need to enable default implementation of Ownable via `openbrush` features.

```toml
openbrush = { version = "~2.1.0", default-features = false, features = ["ownable"] }
```

## Step 2: Add imports and enable unstable feature

Use `openbrush::contract` macro instead of `ink::contract`. Import **everything** from `openbrush::contracts::ownable`.

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_ownable {
    use openbrush::{
        contracts::ownable::*,
        modifiers,
    };
    use ink_storage::traits::SpreadAllocate;
...
```

## Step 3: Define storage

Declare storage struct and declare the field related to `OwnableStorage` trait. Then you need to derive the `OwnableStorage` trait and mark the corresponding field with the `#[OwnableStorageField]` attribute. Deriving this trait allows you to reuse the default implementation of `Ownable`.

```rust
#[ink(storage)]
#[derive(Default, SpreadAllocate, OwnableStorage)]
pub struct MyOwnable {
    #[OwnableStorageField]
    ownable: OwnableData,
}
```

## Step 4: Inherit logic

Inherit implementation of the `Ownable` trait. You can customize (override) methods in this `impl` block.

```rust
impl Ownable for MyOwnable {}
```

## Step 5: Define constructor

Define the constructor and initialize the owner with the contract initiator. Your basic version of `Ownable` contract is ready!

```rust
impl MyOwnable {
    #[ink(constructor)]
    pub fn new() -> Self {
        ink_lang::codegen::initialize_contract(|instance: &mut Self| {
            let caller = instance.env().caller();
            instance._init_with_owner(caller);
        })
    }
}
```

## Step 6: Customize your contract

Customize it by adding ownable logic. We will add a `owner_function` to `MyOwnable` implemenation and add the `only_owner` modifier, which will verify that the caller of the function is the owner.

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_ownable {
    use openbrush::{
        contracts::ownable::*,
        modifiers,
    };
    use ink_storage::traits::SpreadAllocate;


   #[ink(storage)]
   #[derive(Default, SpreadAllocate, OwnableStorage)]
   pub struct MyOwnable {
      #[OwnableStorageField]
      ownable: OwnableData,
   }

   impl Ownable for MyOwnable {}
    
   impl MyOwnable {
      
      #[ink(constructor)]
      pub fn new() -> Self {
        ink_lang::codegen::initialize_contract(|instance: &mut Self| {
            let caller = instance.env().caller();
            instance._init_with_owner(caller);
        })
      }

      #[ink(message)]
      #[modifiers(only_owner)]
      pub fn owner_function(&mut self) -> Result<(), OwnableError> {
         todo!()
      }
   }
}

```

You can check an example of the usage of [Ownable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/ownable).
