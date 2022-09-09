---
sidebar_position: 6
title: Pausable
---

This example shows how you can reuse the implementation of
[pausable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/security/pausable) in `Flipper` contract to `flip` only if the contract is not paused.

## Step 1: Import default implementation

With [default `Cargo.toml`](/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush),
you need to import the `pausable` module, enable the corresponding feature, and embed the module data structure
as described in [that section](/smart-contracts/overview#reuse-implementation-of-traits-from-openbrush).

The main trait is `Pausable`.

## Step 2: Define constructor

Define constructor with default value(paused variable is `false` in that case).

```rust
impl Contract {
   #[ink(constructor)]
   pub fn new() -> Self {
      Self::default()
   }
}
```

## Step 3: Customize your contract

Customize it by adding flipper logic. We will implement `flip` method marked with `when_not_paused` modifier.

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_pausable {
    use ink_storage::traits::SpreadAllocate;
    use openbrush::contracts::pausable::*;
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct Contract {
        #[storage_field]
        pause: pausable::Data,
        flipped: bool,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self| {})
        }

        #[ink(message)]
        #[openbrush::modifiers(when_not_paused)]
        pub fn flip(&mut self) -> Result<(), PausableError> {
            self.flipped = !self.flipped;
            Ok(())
        }

        #[ink(message)]
        pub fn pause(&mut self) -> Result<(), PausableError> {
            self._pause()
        }

        #[ink(message)]
        pub fn unpause(&mut self) -> Result<(), PausableError> {
            self._unpause()
        }
    }

    impl Pausable for Contract {}
}
```

You can check an example of the usage of [Pausable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/pausable).
