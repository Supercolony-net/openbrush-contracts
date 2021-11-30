---
sidebar_position: 7
title: Storage trait
---

In our traits file, we will be storing the data related to our lending smart contract. We will define a struct with the data we want to store, declare a storage trait with `declare_storage_trait` macro and then define the trait.

## Imports

First, we will import the stuff we will be using in our storage.

```rust
use crate::errors::LendingError;
use brush::{
    declare_storage_trait,
    traits::{
        AccountId,
        AccountIdExt,
        Balance,
        InkStorage,
        ZERO_ADDRESS,
    },
};
use ink_storage::{
    collections::HashMap as StorageHashMap,
    traits::SpreadLayout,
};
pub use lending_derive::LendingStorage;
use psp22::traits::PSP22Wrapper;
```

## Define the storage struct

We will define the storage struct and use the `declare_storage_trait` macro which will define the storage trait that we will use in our lending contract.

```rust
#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct LendingData {
    pub assets_lended: StorageHashMap<AccountId, AccountId>,
    pub asset_shares: StorageHashMap<AccountId, AccountId>,
}

declare_storage_trait!(LendingStorage, LendingData);
```

## Define the storage trait

Once we have our storage struct, we will implement the trait, in which we will define getters and setters for our contract's data.

```rust
#[brush::trait_definition]
pub trait LendingStorageTrait: LendingStorage {
    #[ink(message)]
    fn total_asset(&self, asset_address: AccountId) -> Result<Balance, LendingError> {
        let mapped_asset = self
            .get()
            .assets_lended
            .get(&asset_address)
            .cloned()
            .unwrap_or(ZERO_ADDRESS.into());
        if mapped_asset.is_zero() {
            return Err(LendingError::AssetNotSupported)
        }
        let contract = Self::env().account_id();
        let available = PSP22Wrapper::balance_of(&asset_address, contract);
        let unavailable = PSP22Wrapper::balance_of(&mapped_asset, contract);
        Ok(available + unavailable)
    }

    fn total_shares(&self, asset_address: AccountId) -> Result<Balance, LendingError> {
        let mapped_asset = self
            .get()
            .asset_shares
            .get(&asset_address)
            .cloned()
            .unwrap_or(ZERO_ADDRESS.into());
        if mapped_asset.is_zero() {
            return Err(LendingError::AssetNotSupported)
        }
        Ok(PSP22Wrapper::total_supply(&mapped_asset))
    }
}
```