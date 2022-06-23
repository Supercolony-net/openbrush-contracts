---
sidebar_position: 6
title: PSP22 Pausable
---

This example shows how you can implement a [PSP22](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp22) contract with a [Pausable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/security/pausable) extension. See an example of [PSP22Pausable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp22_extensions/pausable) implementation.

## Step 1: Include dependencies

Include `openbrush` as dependency in the cargo file or you can use [default `Cargo.toml`](/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush) template.
After you need to enable default implementation of PSP22 and Pausable via `openbrush` features.

```toml
openbrush = { version = "~2.1.0", default-features = false, features = ["psp22", "pausable"] }
```

## Step 2: Add imports and enable unstable feature

Use `openbrush::contract` macro instead of `ink::contract`. Import **everything** from `openbrush::contracts::psp22` and `openbrush::contracts::pausable`.

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22_pausable {
    use openbrush::{
        contracts::{
            pausable::*,
            psp22::*,
        },
        modifiers,
    };
    use ink_storage::traits::SpreadAllocate;
```

## Step 3: Define storage

Declare the storage struct and declare the fields related to the `PausableStorage` and `PSP22Storage` traits. Then you need to derive the `PausableStorage` and `PSP22Storage` traits and mark the corresponding fields with the `#[PausableStorageField]` and `#[PSP22StorageField]` attributes. Deriving these traits allows you to reuse the `PSP22` implementation with a `Pausable` extension.

```rust
#[ink(storage)]
#[derive(Default, SpreadAllocate, PSP22Storage, PausableStorage)]
pub struct MyPSP22Pausable {
    #[PSP22StorageField]
    psp22: PSP22Data,
    #[PausableStorageField]
    pause: PausableData,
}
```

## Step 4: Inherit logic and implement Pausable logic

Inherit the implementation of the `PSP22` and `Pausable` traits. You can customize (override) methods in this `impl` block. We will implement the `Pausable` logic in this section.

```rust
impl PSP22 for MyPSP22Pausable {}

impl PSP22Transfer for MyPSP22Pausable {
    /// Return `Paused` error if the token is paused
    #[modifiers(when_not_paused)]
    fn _before_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _amount: &Balance,
    ) -> Result<(), PSP22Error> {
        // TODO logic for before token transfer
        Ok(())
    }
}

impl Pausable for MyPSP22Pausable {}
```

## Step 5: Define constructor

Define constructor and add contract functions for pausing and unpausing the contract. Your `PSP22Pausable` contract is ready!

```rust
impl MyPSP22Pausable {
    #[ink(constructor)]
    pub fn new(total_supply: Balance) -> Self {
        ink_lang::codegen::initialize_contract(|instance: &mut Self| {
            assert!(instance._mint(Self::env().caller(), total_supply).is_ok());
        })
    }

    /// Function which changes state to unpaused if paused and vice versa
    #[ink(message)]
    pub fn change_state(&mut self) -> Result<(), PSP22Error> {
        if self.paused() {
            self._unpause()
        } else {
            self._pause()
        }
    }
}
```

You can check an implementation example of [PSP22 Pausable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp22_extensions/pausable).

You can also check the documentation for the basic implementation of [PSP22](/smart-contracts/PSP22).
