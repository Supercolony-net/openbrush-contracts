---
sidebar_position: 7
title: PSP22 Capped
---

This example shows how you can implement a [PSP22](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp22) contract with a supply cap, analogue to [ERC20Capped](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/token/ERC20/extensions/ERC20Capped.sol).

## Step 1: Include dependencies

Include `openbrush` as dependency in the cargo file or you can use [default `Cargo.toml`](/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush) template.
After you need to enable default implementation of PSP22 via `openbrush` features.

```toml
openbrush = { version = "~2.1.0", default-features = false, features = ["psp22"] }
```

## Step 2: Add imports and enable unstable feature

Use `openbrush::contract` macro instead of `ink::contract`. Import **everything** from `openbrush::contracts::psp22`.

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22_capped {
    use ink_prelude::string::String;
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::psp22::extensions::{
            capped::*,
            mintable::*,
        },
        traits::Storage,
    };

```

## Step 3: Define storage

Declare storage struct and declare the field related to the metadata module data structure.
Then you need to derive the `Storage` trait and mark the corresponding field with
the `#[storage_field]` attribute. Deriving this trait allows you to reuse the
`PSP22Capped` extension in your `PSP22` implementation.

```rust
#[ink(storage)]
#[derive(Default, SpreadAllocate, Storage)]
pub struct Contract {
    #[storage_field]
    psp22: psp22::Data,
    #[storage_field]
    cap: Data,
}
```

## Step 4: Define constructor and contract functions

Define constructor, inherit `PSP22`, and override the basic functions for capped implementation. Your `PSP22Capped` contract is ready!

```rust
impl PSP22 for Contract {}

impl PSP22Capped for Contract {}

impl PSP22Mintable for Contract {}

impl psp22::Transfer for Contract {
    fn _before_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _amount: &Balance,
    ) -> Result<(), PSP22Error> {
        // `is_none` means that it is minting
        if _from.is_none() && (self.total_supply() + _amount) > self.cap() {
            return Err(PSP22Error::Custom(String::from("Cap exceeded")))
        }
        Ok(())
    }
}

impl Contract {
    /// Constructor which mints `initial_supply` of the token to sender
    /// Will set the token's cap to `cap`
    #[ink(constructor)]
    pub fn new(inital_supply: Balance, cap: Balance) -> Self {
        ink_lang::codegen::initialize_contract(|instance: &mut Self| {
            assert!(instance._init_cap(cap).is_ok());
            assert!(instance.mint(instance.env().caller(), inital_supply).is_ok());
        })
    }
}
```

You can check an implementation example of [PSP22 Capped](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp22_extensions/capped).

You can also check the documentation for the basic implementation of [PSP22](/smart-contracts/PSP22).
