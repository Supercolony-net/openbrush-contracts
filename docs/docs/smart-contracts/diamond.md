---
sidebar_position: 4
title: Diamond Standard
---

This example shows how you can use the implementation of [diamond standard](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/upgradability/diamond) to implement diamond standard pattern for upgradeable and unlimited contracts.

## Step 1: Include dependencies

Include `openbrush` as dependency in the cargo file or you can use [default `Cargo.toml`](/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush) template.
After you need to enable default implementation of Diamond Standard via `openbrush` features.

```toml
openbrush = { version = "~2.1.0", default-features = false, features = ["diamond"] }
```

## Step 2: Add imports and enable unstable feature

Use `openbrush::contract` macro instead of `ink::contract`. Import **everything** from `openbrush::contracts::ownable` and `openbrush::contracts::diamond`

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_diamond {
    use openbrush::{
        contracts::{
            ownable::*,
            diamond::*,
        },
        modifiers,
    };
...
```

## Step 3: Define storage

Declare storage struct and declare the field related to `DiamondStorage` trait. Then you need to derive the `DiamondStorage` trait and mark the corresponding field with the `#[DiamondStorageField]` attribute. Deriving this trait allows you to reuse the default implementation of `DiamondStandard`.

```rust
#[ink(storage)]
#[derive(Default, SpreadAllocate, DiamondStorage)]
pub struct DiamondContract {
    #[DiamondStorageField]
    diamond: DiamondData,
}
```

## Step 4: Inherit logic

Inherit implementation of the `Diamond` trait and of the `Ownable` trait. You can customize (override) methods in this `impl` block.

```rust
impl Ownable for DiamondContract {}

impl Diamond for DiamondContract {}
```

## Step 5: Define constructor

Define the constructor and initialize the owner with the contract initiator. Your basic version of `Diamond` contract is ready!

```rust
impl DiamondContract {
    #[ink(constructor)]
    pub fn new(owner: AccountId, diamond_hash: Hash) -> Self {
        ink_lang::codegen::initialize_contract(|instance: &mut Self| {
            instance._init_with_owner(owner);
            instance.diamond.self_hash = diamond_hash;
        })
    }
}
```

## Step 6: Define forward function

Define the forward function to make delegate calls of facet contracts through the diamond contract.

```rust
impl DiamondContract {
    #[ink(message, payable, selector = _)]
    pub fn forward(&self) {
        self._fallback();
    }
}
```
## Step 6: Customize your contract

You can add more basic functionality for your diamond contract by adding functions to `DiamondContract` implemenation, but the point of the Diamond standard is not to increase the size of your contract, and to add upgradeable functionality to your contract via so called facets.

When you create a new contract (facet), which you want to make delegate calls from your diamond contract to, you will call the `diamond_cut` function on your diamond contract, with the code hash of your new facet and the selectors of all the functions from this facet you want to use. The diamond will register them and anytime you call this function on your diamond contract, it will make the delegate call to the facet the function belongs to. You can add, remove or replace these functions anytime with the `diamond_cut` function, some of the limitations are, that you can not add functions with the same selectors, when replacing functions, the new function needs to be from a different contract, then currently in use, and when removing functions, the function needs to be registered in the diamond contract.

You can check an example of the usage of [Diamond](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/diamond).
