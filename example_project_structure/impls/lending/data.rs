// importing everything publicly from traits allows you to import every stuff related to lending
// by one import
pub use crate::traits::lending::*;
use ink_storage::traits::{
    SpreadAllocate,
    SpreadLayout,
};
use openbrush::{
    declare_storage_trait,
    storage::{
        Mapping,
        TypeGuard,
    },
    traits::{
        AccountId,
        AccountIdExt,
        Balance,
        Hash,
        ZERO_ADDRESS,
    },
};
// it is public because when you will import the trait you also will import the derive for the trait
pub use lending_project_derive::LendingStorage;

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadAllocate, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
/// define the struct with the data that our smart contract will be using
/// this will isolate the logic of our smart contract from its storage
pub struct LendingData {
    /// mapping from asset address to lended asset address
    /// when X amount of asset is lended, X amount of asset it is mapped to is minted
    /// so the contract knows how much of asset it has and how much of the asset was lended
    pub assets_lended: Mapping<AccountId, AccountId>,
    /// mapping from asset address to shares asset address
    /// the lended asset is mapped to a shares asset which represents
    /// the total share of the mapping asset
    /// example: if a user has X% of the total supply of the asset A', they
    /// are eligible to withdraw X% of the asset A tracked by this contract
    pub asset_shares: Mapping<AccountId, AccountId>,
    /// mapping from share token to asset token
    pub shares_asset: Mapping<AccountId, AccountId>,
    /// mapping from asset address to bool
    /// maps to `true` if an asset is accepted for using as collateral
    pub collateral_accepted: Mapping<AccountId, bool>,
    /// mapping from tuple of two assets to balance
    /// mapped balance represents the amount of assets of tuple.1 we get
    /// when we deposit 1 unit of tuple.0
    /// we are using this just to simulate an oracle in our example
    /// in the example the returned balance will be amount of stable coin for an asset
    pub asset_price: Mapping<(AccountId, AccountId), Balance, AssetPriceKey>,
    /// code hash of the `SharesContract`
    pub shares_contract_code_hash: Hash,
    /// the `AccountId` of the `Loan`
    pub loan_account: AccountId,
}

pub struct AssetPriceKey;

impl<'a> TypeGuard<'a> for AssetPriceKey {
    type Type = &'a (&'a AccountId, &'a AccountId);
}

declare_storage_trait!(LendingStorage);

/// this internal function will be used to set price of `asset_in` when we deposit `asset_out`
/// we are using this function in our example to simulate an oracle
pub fn set_asset_price<T>(instance: &mut T, asset_in: &AccountId, asset_out: &AccountId, price: &Balance)
where
    T: LendingStorage<Data = LendingData>,
{
    instance.get_mut().asset_price.insert(&(asset_in, asset_out), price);
}

/// this internal function will be used to set price of `asset_in` when we deposit `asset_out`
/// we are using this function in our example to simulate an oracle
pub fn get_asset_price<T>(instance: &T, amount_in: &Balance, asset_in: &AccountId, asset_out: &AccountId) -> Balance
where
    T: LendingStorage<Data = LendingData>,
{
    let price = instance.get().asset_price.get(&(asset_in, asset_out)).unwrap_or(0);
    price * amount_in
}

/// Internal function which will return the address of the shares token
/// which are minted when `asset_address` is borrowed
pub fn get_reserve_asset<T>(instance: &T, asset_address: &AccountId) -> Result<AccountId, LendingError>
where
    T: LendingStorage<Data = LendingData>,
{
    let reserve_asset = instance
        .get()
        .asset_shares
        .get(&asset_address)
        .unwrap_or(ZERO_ADDRESS.into());
    if reserve_asset.is_zero() {
        return Err(LendingError::AssetNotSupported)
    }
    Ok(reserve_asset)
}

/// internal function which will return the address of asset
/// which is bound to `shares_address` shares token
pub fn get_asset_from_shares<T>(instance: &T, shares_address: &AccountId) -> Result<AccountId, LendingError>
where
    T: LendingStorage<Data = LendingData>,
{
    let token = instance
        .get()
        .shares_asset
        .get(shares_address)
        .unwrap_or(ZERO_ADDRESS.into());
    if token.is_zero() {
        return Err(LendingError::AssetNotSupported)
    }
    Ok(token)
}
