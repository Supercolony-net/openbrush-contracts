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
use psp22::traits::PSP22Ref;
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
    pub shares_asset: StorageHashMap<AccountId, AccountId>,
    pub collateral_accepted: StorageHashMap<AccountId, bool>,
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
        todo()!;
    }

    #[ink(message)]
    fn total_shares(&self, asset_address: AccountId) -> Result<Balance, LendingError> {
        todo()!;
    }

    #[ink(message)]
    fn is_accepted_lending(&self, asset_address: AccountId) -> bool {
        todo()!;
    }

    #[ink(message)]
    fn is_accepted_collateral(&self, asset_address: AccountId) -> bool {
        todo()!;
    }

    fn _accept_lending(&mut self, asset_address: AccountId, share_address: AccountId, reserve_address: AccountId) {
        todo()!;
    }

    fn _disallow_lending(&mut self, asset_address: AccountId) {
        todo()!;
    }

    fn _set_collateral_accepted(&mut self, asset_address: AccountId, accepted: bool) {
        todo()!;
    }

    fn _set_asset_price(&mut self, asset_in: AccountId, asset_out: AccountId, price: Balance) {
        todo()!;
    }

    fn _get_asset_price(&self, amount_in: Balance, asset_in: AccountId, asset_out: AccountId) -> Balance {
        todo()!;
    }

    fn _get_reserve_asset(&self, asset_address: AccountId) -> AccountId {
        todo()!;
    }

    fn _get_asset_from_shares(&self, shares_address: AccountId) -> AccountId {
        todo()!;
    }
}
```