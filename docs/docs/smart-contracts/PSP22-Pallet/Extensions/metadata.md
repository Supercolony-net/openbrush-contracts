---
sidebar_position: 1
title: PSP22 Metadata
---

This example shows how you can reuse the implementation of [PSP22](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp22_pallet) token with the [PSP22Metadata](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp22_pallet/extensions/metadata.rs) extension via `pallet-assets` chain extension.

First, you should implement basic version of [PSP22 Pallet](/smart-contracts/PSP22-Pallet).

## Step 1: Add imports and enable unstable feature

Use `openbrush::contract` macro instead of `ink::contract`. Import **everything** from `openbrush::contracts::psp22_pallet::extensions::metadata`.

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22_pallet {
    use openbrush::contracts::psp22_pallet::extensions::metadata::*;
```

## Step 2: Define storage

Declare storage struct and declare the field related to the metadata module data structure.
Then you need to derive the `Storage` trait and mark the corresponding field with
the `#[storage_field]` attribute. Deriving this trait allows you to reuse the
`PSP22Metadata` extension in your `PSP22 Pallet` implementation.

```rust
#[ink(storage)]
#[derive(Default, SpreadAllocate, Storage)]
pub struct Contract {
    #[storage_field]
    pallet: psp22_pallet::Data,
}
```

## Step 3: Inherit logic

Inherit the implementation of the `PSP22Metadata` trait. You can customize (override)
methods in this `impl` block.

Inherit the implementation of the `PSP22` trait.

```rust
impl PSP22 for Contract {}

impl PSP22Metadata for Contract {}
```

## Step 4: Define constructor

Define constructor. Your `PSP22Metadata` contract is ready!

```rust
impl Contract {
    /// During instantiation of the contract, you need to pass native tokens as a deposit
    /// for asset creation.
    #[ink(constructor)]
    #[ink(payable)]
    pub fn new(asset_id: u32, min_balance: Balance, total_supply: Balance, name: String, symbol: String, decimal: u8) -> Self {
        ink_lang::codegen::initialize_contract(|instance: &mut Contract| {
            // The contract is admin of the asset
            instance
                ._create(asset_id, Self::env().account_id(), min_balance)
                .expect("Should create an asset");
            instance.pallet.asset_id = asset_id;
            instance.pallet.origin = Origin::Caller;
            assert!(instance.pallet.pallet_assets.set_metadata(asset_id, name, symbol, decimal).is_ok());
            instance
                ._mint(instance.env().caller(), total_supply)
                .expect("Should mint");
        })
    }
}
```

## Final code

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22_pallet_metadata {
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::psp22_pallet::extensions::metadata::*,
        traits::Storage,
    };
    use openbrush::traits::String;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct Contract {
        #[storage_field]
        pallet: psp22_pallet::Data,
    }

    impl PSP22 for Contract {}

    impl PSP22Metadata for Contract {}

    impl Contract {
        /// During instantiation of the contract, you need to pass native tokens as a deposit
        /// for asset creation.
        #[ink(constructor)]
        #[ink(payable)]
        pub fn new(asset_id: u32, min_balance: Balance, total_supply: Balance, name: String, symbol: String, decimal: u8) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Contract| {
                // The contract is admin of the asset
                instance
                    ._create(asset_id, Self::env().account_id(), min_balance)
                    .expect("Should create an asset");
                instance.pallet.asset_id = asset_id;
                instance.pallet.origin = Origin::Caller;
                assert!(instance.pallet.pallet_assets.set_metadata(asset_id, name, symbol, decimal).is_ok());
                instance
                    ._mint(instance.env().caller(), total_supply)
                    .expect("Should mint");
            })
        }
    }
}
```

You can also check the documentation for the basic implementation of [PSP22 Pallet](/smart-contracts/PSP22-Pallet).
