---
sidebar_position: 7
title: PSP22 Capped
---

This example shows how you can reuse the implementation of [PSP22](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp22) token with the [PSP22Capped](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp22/extensions/capped.rs) extension.

First, you should implement basic version of [PSP22](/smart-contracts/PSP22).

## Step 1: Add imports and enable unstable feature

Use `openbrush::contract` macro instead of `ink::contract`. Import **everything** from `openbrush::contracts::psp22::extensions::capped`.

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22 {
    use openbrush::contracts::psp22::extensions::capped::*;
```

## Step 2: Define storage

Declare storage struct and declare the field related to the capped module data structure.
Then you need to derive the `Storage` trait and mark the corresponding field with
the `#[storage_field]` attribute. Deriving this trait allows you to reuse the
`PSP22Capped` extension in your `PSP22` implementation.

```rust
#[ink(storage)]
#[derive(Default, SpreadAllocate, Storage)]
pub struct Contract {
    ...
    #[storage_field]
    cap: capped::Data,
}
```

## Step 3: Inherit logic

Inherit the implementation of the `PSP22Capped` trait. You can customize (override)
methods in this `impl` block.

Inherit the implementation of the `PSP22` trait.

```rust
impl PSP22 for Contract {}

impl PSP22Capped for Contract {}
```

Override `psp22::Transfer` to check is the cap exceeded before minting.

```rust 
impl psp22::Transfer for Contract {
        fn _before_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
            _to: Option<&AccountId>,
            _amount: &Balance,
        ) -> Result<(), PSP22Error> {
            // `is_none` means that it is minting
            if _from.is_none() && self._is_cap_exceeded(_amount) {
                return Err(PSP22Error::Custom(String::from("Cap exceeded")))
            }
            Ok(())
        }
    }
```

## Step 4: Define constructor

Define constructor. Your `PSP22Capped` contract is ready!

```rust
impl Contract {
    #[ink(constructor)]
    pub fn new(inital_supply: Balance) -> Self {
        ink_lang::codegen::initialize_contract(|instance: &mut Self| {
            assert!(instance._init_cap(cap).is_ok());
            assert!(instance.mint(instance.env().caller(), inital_supply).is_ok());
        })
    }
}
```

## Final code

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22 {
    use ink::storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::psp22::extensions::{
            capped::*,
        },
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct Contract {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        cap: capped::Data,
    }

    impl PSP22 for Contract {}

    impl PSP22Capped for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new(inital_supply: Balance, cap: Balance) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                assert!(instance._init_cap(cap).is_ok());
                assert!(instance.mint(instance.env().caller(), inital_supply).is_ok());
            })
        }
    }
}
```

You can check an implementation example of [PSP22 Capped](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp22_extensions/capped).

You can also check the documentation for the basic implementation of [PSP22](/smart-contracts/PSP22).
