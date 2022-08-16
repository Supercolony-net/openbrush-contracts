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
    use openbrush::contracts::psp22::*;
    use ink_prelude::string::String;
    use ink_storage::traits::SpreadAllocate;
...
```

## Step 3: Define storage

Declare the storage struct and the field related to the `PSP22Storage` trait, derive the `PSP22Storage` trait and mark the corresponding field with the `#[PSP22StorageField]` attribute. Also add the storage variable for cap.

```rust
#[ink(storage)]
#[derive(Default, SpreadAllocate, PSP22Storage)]
pub struct MyPSP22Capped {
    #[PSP22StorageField]
    psp22: PSP22Data,
    cap: Balance,
}
```

## Step 4: Define constructor and contract functions

Define constructor, inherit `PSP22`, and override the basic functions for capped implementation. Your `PSP22Capped` contract is ready!

```rust
impl PSP22 for MyPSP22Capped {}

impl MyPSP22Capped {
    /// Constructor which mints `initial_supply` of the token to sender
    /// Will set the token's cap to `cap`
    #[ink(constructor)]
    pub fn new(inital_supply: Balance, cap: Balance) -> Self {
        ink_lang::codegen::initialize_contract(|instance: &mut Self| {
            assert!(instance.init_cap(cap).is_ok());
            assert!(instance._mint(instance.env().caller(), inital_supply).is_ok());
        })
    }

    /// Expose the `_mint` function
    #[ink(message)]
    pub fn mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        self._mint(account, amount)
    }

    #[ink(message)]
    /// Returns the token's cap
    pub fn cap(&self) -> Balance {
        self.cap
    }

    /// Overrides the `_mint` function to check for cap overflow before minting tokens
    /// Performs `PSP22::_mint` after the check succeeds
    fn _mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        if (self.total_supply() + amount) > self.cap() {
            return Err(PSP22Error::Custom(String::from("Cap exceeded")))
        }
        PSP22Internal::_mint(self, account, amount)
    }

    /// Initializes the token's cap
    fn init_cap(&mut self, cap: Balance) -> Result<(), PSP22Error> {
        if cap <= 0 {
            return Err(PSP22Error::Custom(String::from("Cap must be above 0")))
        }
        self.cap = cap;
        Ok(())
    }
}
```

You can check an implementation example of [PSP22 Capped](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp22_extensions/capped).

You can also check the documentation for the basic implementation of [PSP22](/smart-contracts/PSP22).
