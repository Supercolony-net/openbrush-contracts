---
sidebar_position: 6
title: PSP22 Pausable
---

This example shows how you can implement a [PSP22](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp22) contract with a [Pausable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/security/pausable) extension. See an example of [PSP22Pausable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp22_extensions/pausable) implementation.

First, you should implement basic version of [PSP22](/smart-contracts/PSP22).

## Step 1: Import default implementation

With [default `Cargo.toml`](/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush),
you need to import the `psp22` and `pausable` modules, enable corresponding features, and embed modules data structures
as described in [that section](/smart-contracts/overview#reuse-implementation-of-traits-from-openbrush).

The main trait is `PSP22` and `Pausable`.

## Step 2: Inherit logic and apply `when_not_paused` modifier

Inherit the implementation of the `PSP22` and `Pausable` traits. 
You can customize (override) methods in this `impl` block. We will apply the 
`when_not_paused` modifier for the transfer.

```rust
impl PSP22 for Contract {}

impl Transfer for Contract {
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

impl Pausable for Contract {}
```

## Step 3: Define constructor

Define constructor and add contract functions for pausing and unpausing the contract.

```rust
impl Contract {
    #[ink(constructor)]
    pub fn new(total_supply: Balance) -> Self {
        ink::codegen::initialize_contract(|instance: &mut Self| {
            assert!(instance._mint_to(Self::env().caller(), total_supply).is_ok());
        })
    }
}
```

## Step 4: Customize your contract with `Pausable` logic

Add the `change_state` function that allow switch pause state.

```rust
impl Contract {
    ...
    
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

## Final code:

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
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        pause: pausable::Data,
    }

    impl PSP22 for Contract {}

    impl Transfer for Contract {
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

    impl Pausable for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            ink::codegen::initialize_contract(|instance: &mut Self| {
                assert!(instance._mint_to(Self::env().caller(), total_supply).is_ok());
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
}
```

You can check an implementation example of [PSP22 Pausable](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp22_extensions/pausable).

You can also check the documentation for the basic implementation of [PSP22](/smart-contracts/PSP22).
