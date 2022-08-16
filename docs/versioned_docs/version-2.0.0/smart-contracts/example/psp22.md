---
sidebar_position: 3
title: Implement PSP-22 contract
---

First, we will cover the implementation of [PSP-22](/smart-contracts/PSP22) 
token used by our smart contract, which will represent the stable coin that we will be 
lending and another [PSP-22](/smart-contracts/PSP22) token which we will be 
using as collateral. These are used just to test our example, you will not be creating 
an actual [PSP-22](/smart-contracts/PSP22) implementation of stable coin or collateral 
token in your lending protocol, but this will also showcase how to implement 
a basic implementation of a fungible token with OpenBrush.

## Definition of the `StableCoin` trait

In the `traits/stable_coin.rs`, we will define a `StableCoin` trait.
That trait contains only two super traits: `PSP22` and `PSP22Metadata`, without any other method.
That shows that `StableCoin` is simple `PSP22`. In the implementation of the contract
we will implement that trait to be sure that all super traits are also implemented.
`StableCoinRef` can be used by other developers to do a cross contract call to `StableCoinContract`.

```rust
use openbrush::contracts::traits::psp22::{
    extensions::metadata::*,
    *,
};

#[openbrush::wrapper]
pub type StableCoinRef = dyn PSP22 + PSP22Metadata;

#[openbrush::trait_definition]
pub trait StableCoin: PSP22 + PSP22Metadata {}
```

## Add dependencies

First we will add the dependencies used in our [PSP-22](/smart-contracts/PSP22)
contract to the `Cargo.toml` file. You will import the same dependencies as in 
the [PSP-22](/smart-contracts/PSP22) documentation, so we will not show 
it here to keep it simple.

## Implement the contract

We want a basic [PSP-22](/smart-contracts/PSP22) token with metadata, 
so we will add the [PSP-22 Metadata](/smart-contracts/PSP22/extensions/metadata) 
extension to our contract. We will add a `openbrush::contract` macro to our contract 
and add some imports:

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

/// This is a simple `PSP-22` which will be used as a stable coin and a collateral token in our lending contract
#[openbrush::contract]
pub mod token {
    use openbrush::contracts::psp22::extensions::metadata::*;
    use ink_prelude::string::String;
    use lending_project::traits::stable_coin::*;
    use ink_storage::traits::SpreadAllocate;
```

## Define the storage

We will derive the storage traits related to `PSP-22` and `PSP-22 Metadata` and declare the fields related to these traits.

```rust
/// Define the storage for PSP22 data and Metadata data
#[ink(storage)]
#[derive(Default, SpreadAllocate, PSP22Storage, PSP22MetadataStorage)]
pub struct StableCoinContract {
    #[PSP22StorageField]
    psp22: PSP22Data,
    #[PSP22MetadataStorageField]
    metadata: PSP22MetadataData,
}
```

## Implement the PSP22 and PSP22Metadata traits and define the constructor

We will implement the `PSP22Metadata` trait and define the constructor where we 
will set the `name` and the `symbol` for our token. Also, we will mint the 
initial supply of tokens to the caller of the constructor.

```rust
/// implement PSP22 Trait for our coin
impl PSP22 for StableCoinContract {}

/// implement PSP22Metadata Trait for our coin
impl PSP22Metadata for StableCoinContract {}

// It forces the compiler to check that you implemented all super traits
impl StableCoin for StableCoinContract {}

impl StableCoinContract {
    /// constructor with name and symbol
    #[ink(constructor)]
    pub fn new(name: Option<String>, symbol: Option<String>) -> Self {
        ink_lang::codegen::initialize_contract(|instance: &mut StableCoinContract| {
            instance.metadata.name = name;
            instance.metadata.symbol = symbol;
            instance.metadata.decimals = 18;
            let total_supply = 1_000_000 * 10_u128.pow(18);
            assert!(instance._mint(instance.env().caller(), total_supply).is_ok());
        })
    }
}
```