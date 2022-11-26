---
sidebar_position: 3
title: Ownable
---

This example shows how you can use the implementation of [ownable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/access/ownable) to provide `only owner` rights for contract's functions.

## Step 1: Import default implementation

With [default `Cargo.toml`](/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush),
you need to import the `ownable` module, enable the corresponding feature, and embed the module data structure 
as described in [that section](/smart-contracts/overview#reuse-implementation-of-traits-from-openbrush).

The main trait is `Ownable`.

## Step 2: Define constructor

Define the constructor and initialize the owner with the contract initiator.

```rust
impl Contract {
    #[ink(constructor)]
    pub fn new() -> Self {
        ink::codegen::initialize_contract(|instance: &mut Self| {
            let caller = instance.env().caller();
            instance._init_with_owner(caller);
        })
    }
}
```

## Step 3: Customize your contract

Customize it by adding ownable logic. We will add a `owner_function` to `MyOwnable` implemenation 
and add the `only_owner` modifier, which will verify that the caller of the function is the owner.

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_ownable {
    use openbrush::{
        contracts::ownable::*,
        modifiers,
    };
    use openbrush::traits::Storage;


   #[ink(storage)]
   #[derive(Default, Storage)]
   pub struct Contract {
      #[storage_field]
      ownable: ownable::Data,
   }

   impl Ownable for Contract {}
    
   impl Contract {
      #[ink(constructor)]
      pub fn new() -> Self {
        ink::codegen::initialize_contract(|instance: &mut Self| {
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
