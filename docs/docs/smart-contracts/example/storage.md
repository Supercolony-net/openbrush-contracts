---
sidebar_position: 8
title: Storage trait
---

In our traits file, we will be storing the data related to our lending smart contract. We will define a struct with the data we want to store, declare a storage trait with `declare_storage_trait` macro and then define the trait. In this example we will not be using price oracles, we will do our own simulated oracle. Since oracles are not the point of this example, it will be enough for us. We will store prices info in our storage contract and then define some more stuff later.

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
    pub collateral_accepted: StorageHashMap<AccountId, bool>,
    // for our oracle simulation
    pub asset_price: StorageHashMap<(AccountId, AccountId), Balance>,
}

declare_storage_trait!(LendingStorage, LendingData);
```

## Define the storage trait

Once we have our storage struct, we will implement the trait, in which we will define getters and setters for our contract's data (including the simulated oracle data).

```rust
#[brush::trait_definition]
pub trait LendingStorageTrait: LendingStorage {
    #[ink(message)]
    fn total_asset(&self, asset_address: AccountId) -> Result<Balance, LendingError> {
        // get asset from mapping
        let mapped_asset = self
            .get()
            .assets_lended
            .get(&asset_address)
            .cloned()
            .unwrap_or(ZERO_ADDRESS.into());
        // return error if the asset is not supported
        if mapped_asset.is_zero() {
            return Err(LendingError::AssetNotSupported)
        }
        let contract = Self::env().account_id();
        let available = PSP22Ref::balance_of(&asset_address, contract);
        let unavailable = PSP22Ref::balance_of(&mapped_asset, contract);
        Ok(available + unavailable)
    }

    /// this function will return the total amount of shares minted for an asset
    ///
    /// Returns `AssetNotSupported` error if we try to get shares of asset not supported by our contract
    #[ink(message)]
    fn total_shares(&self, asset_address: AccountId) -> Result<Balance, LendingError> {
        // get asset from mapping
        let mapped_asset = self
            .get()
            .asset_shares
            .get(&asset_address)
            .cloned()
            .unwrap_or(ZERO_ADDRESS.into());
        // return error if the asset is not supported
        if mapped_asset.is_zero() {
            return Err(LendingError::AssetNotSupported)
        }
        Ok(PSP22Ref::total_supply(&mapped_asset))
    }

    #[ink(message)]
    fn is_accepted_collateral(&self, asset_address: AccountId) -> bool {
        self.get()
            .collateral_accepted
            .get(&asset_address)
            .cloned()
            .unwrap_or(false)
    }

    fn _accept_lending(&mut self, asset_address: AccountId, share_address: AccountId, reserve_address: AccountId) {
        self.get_mut().asset_shares.insert(asset_address, share_address);
        self.get_mut().assets_lended.insert(asset_address, reserve_address);
    }

    fn _accept_collateral(&mut self, asset_address: AccountId) {
        self.get_mut().collateral_accepted.insert(asset_address, true);
    }

    fn _accept_lending(&mut self, asset_address: AccountId, share_address: AccountId, reserve_address: AccountId) {
        self.get_mut().asset_shares.insert(asset_address, share_address);
        self.get_mut().assets_lended.insert(asset_address, reserve_address);
    }

    fn _set_asset_price(&mut self, asset_in: AccountId, asset_out: AccountId, price: Balance) {
        self.get_mut().asset_price.insert((asset_in, asset_out), price);
    }

    fn _get_asset_price(&self, amount_in: Balance, asset_in: AccountId, asset_out: AccountId) -> Balance {
        self.get().asset_price.get(&(asset_in, asset_out)).cloned().unwrap_or(0) * amount_in
    }

    fn _get_reserve_asset(&self, asset_address: AccountId) -> AccountId {
        self.get()
            .asset_shares
            .get(&asset_address)
            .cloned()
            .unwrap_or(ZERO_ADDRESS.into())
    }
}
```