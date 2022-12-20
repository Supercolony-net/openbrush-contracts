---
sidebar_position: 9
title: PSP22 Pallet
---

This example shows how you can reuse the implementation of [PSP22 Pallet](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp22_pallet) via `pallet-assets` chain extension. Also, this example shows how you can customize the logic, for example, to get current `asset_id`.

## Step 1: Import default implementation

With [default `Cargo.toml`](/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush),
you need to import the `psp22_pallet` module, enable the corresponding feature, and embed the module data structure
as described in [that section](/smart-contracts/overview#reuse-implementation-of-traits-from-openbrush).

## Step 2: Define storage and implement default PSP22 trait

Use `psp22_pallet` storage and implement `PSP22` trait for your contract.

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22_pallet {
    use openbrush::{
        contracts::psp22_pallet::*,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        pallet: psp22_pallet::Data,
    }

    impl PSP22 for Contract {}
}
```

## Step 3: Add constructor

Add constructor for your contract, create asset and mint tokens to caller.

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22_pallet {
    use openbrush::{
        contracts::psp22_pallet::*,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        pallet: psp22_pallet::Data,
    }

    impl PSP22 for Contract {}

    impl Contract {
        /// During instantiation of the contract, you need to pass native tokens as a deposit
        /// for asset creation.
        #[ink(constructor)]
        #[ink(payable)]
        pub fn new(asset_id: u32, min_balance: Balance, total_supply: Balance) -> Self {
            let mut instance = Self::default();

            instance
                ._create(asset_id, Self::env().account_id(), min_balance)
                .expect("Should create an asset");
            instance.pallet.asset_id = asset_id;
            instance.pallet.origin = Origin::Caller;
            instance
                ._mint_to(Self::env().caller(), total_supply)
                .expect("Should mint");
            
            instance
        }

        /// Asset id of the asset in the `pallet-assets`
        #[ink(message)]
        pub fn asset_id(&self) -> u32 {
            self.pallet.asset_id
        }
    }
}

```

You can check an example of the usage of [PSP22 Pallet](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp22_pallet).

Also you can use extensions for PSP22 token:

[PSP22 Pallet Metadata](Extensions/metadata): metadata for PSP22 Pallet.

[PSP22 Pallet Mintable](Extensions/mintable): creation of new tokens.

[PSP22 Pallet Burnable](Extensions/burnable): destruction of own tokens.
