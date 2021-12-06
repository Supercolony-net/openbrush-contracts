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

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
/// define the struct with the data that our smart contract will be using
/// this will isolate the logic of our smart contract from its storage
pub struct LendingData {
    /// mapping from asset address to lended asset address
    /// when X amount of asset is lended, X amount of asset it is mapped to is minted
    /// so the contract knows how much of asset it has and how much of the asset was lended
    pub assets_lended: StorageHashMap<AccountId, AccountId>,
    /// mapping from asset address to shares asset address
    /// the lended asset is mapped to a shares asset which represents
    /// the total share of the mapping asset
    /// example: if a user has X% of the total supply of the asset A', they
    /// are eligible to withdraw X% of the asset A tracked by this contract
    pub asset_shares: StorageHashMap<AccountId, AccountId>,
}

declare_storage_trait!(LendingStorage, LendingData);

// we will declare a trait which holds getters and setters for our storage struct
#[brush::trait_definition]
pub trait LendingStorageTrait: LendingStorage {
    /// this function will return the total amount of assets available to borrow
    /// along with amount of the same asset borrowed
    ///
    /// Returns `AssetNotSupported` error if we try to get amount of asset not supported by our contract
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

    /// this function will return true if the asset is accepted by the contract
    #[ink(message)]
    fn is_accepted_lending(&self, asset_address: AccountId) -> bool {
        !self
            .get()
            .asset_shares
            .get(&asset_address)
            .cloned()
            .unwrap_or(ZERO_ADDRESS.into())
            .is_zero()
    }

    /// this function will accept `asset_address` for lending
    /// `share_address` is the address of the shares token of the asset being allowed
    /// `reserve_address` is the address of the reserves token of the asset being allowed
    fn _accept_lending(&mut self, asset_address: AccountId, share_address: AccountId, reserve_address: AccountId) {
        self.get_mut().asset_shares.insert(asset_address, share_address);
        self.get_mut().assets_lended.insert(asset_address, reserve_address);
    }
}
