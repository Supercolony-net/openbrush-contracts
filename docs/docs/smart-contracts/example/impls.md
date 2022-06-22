---
sidebar_position: 7
title: Lending impls
---

The lending contract implementation consists of two traits:
- `Lending` - contains methods that can be called by anyone.
  These methods can be used to lend, borrow, liquidate assets and get some information about them.
- `LendingPermissioned` - contains methods with restrictions. These methods can be called by a manager.
  These methods allow defining the list of allowed tokens, setting price and etc.

We will define everything stuff from the previous chapter for "inheritable" contracts:
- Both traits in `traits/lending.rs`
- A data structure and `LendingStorage` storage trait in `impls/lending/data.rs`
- A derive macro for `LendingStorage` in `derive/lib.rs`
- A generic implementation for trait `Lending` in `impls/lending/lending.rs`
- A generic implementation for trait `LendingPermissioned` in `impls/lending/lending_permissioned.rs`

## Definition of traits

In the `traits/lending.rs`, we will define `Lending` and `LendingPermissioned` traits.
We plan that `LendingContract` also will implement `AccessControl` and `Pausable`, so
`LendingContractRef` is defined like a combination of `AccessControl`, `Pausable`, `LendingPermissioned` and `Lending`.
That wrapper describes all methods available in the `LendingContract`.

```rust
use openbrush::{
  contracts::traits::{
    access_control::*,
    pausable::*,
    psp22::PSP22Error,
    psp34::{
      Id,
      PSP34Error,
    },
  },
  traits::{
    AccountId,
    Balance,
  },
};

/// Combination of all traits of the contract to simplify calls to the contract
#[openbrush::wrapper]
pub type LendingContractRef = dyn Lending + LendingPermissioned + AccessControl + Pausable;

#[openbrush::wrapper]
pub type LendingRef = dyn Lending;

#[openbrush::trait_definition]
pub trait Lending {
  /// This function will return the total amount of assets available to borrow
  /// along with amount of the same asset borrowed
  ///
  /// Returns `AssetNotSupported` error if we try to get amount of asset not supported by our contract
  #[ink(message)]
  fn total_asset(&self, asset_address: AccountId) -> Result<Balance, LendingError>;

  /// This function will return the total amount of shares minted for an asset
  ///
  /// Returns `AssetNotSupported` error if we try to get shares of asset not supported by our contract
  #[ink(message)]
  fn total_shares(&self, asset_address: AccountId) -> Result<Balance, LendingError>;

  /// This function will return true if the asset is accepted by the contract
  #[ink(message)]
  fn is_accepted_lending(&self, asset_address: AccountId) -> bool;

  /// This function will return true if the asset is accepted by the contract
  #[ink(message)]
  fn is_accepted_collateral(&self, asset_address: AccountId) -> bool;

  /// This function is called by a user who wants to lend tokens and gain interest
  ///
  /// `asset_address` is the AccountId of the PSP-22 token to be deposited
  /// `amount` is the amount to be deposited
  ///
  /// Returns `InsufficientAllowanceToLend` if the caller does not have enough allowance
  /// Returns `InsufficientBalanceToLend` if the caller does not have enough balance
  /// Returns `AssetNotSupported` if the asset is not supported for lending
  #[ink(message)]
  fn lend_assets(&mut self, asset_address: AccountId, amount: Balance) -> Result<(), LendingError>;

  /// This function is called by a user who wants to borrow tokens. In order to do that,
  /// they need to deposit collateral. The value of borrowed assets will be equal to 70%
  /// of the value of deposited collateral.
  ///
  /// `asset_address` is the AccountId of the PSP-22 token to be borrowed
  /// `collateral_address` is the AccountId of the PSP-22 token used as collateral
  /// `amount` is the amount to be deposited
  ///
  /// Returns `AssetNotSupported` if `asset_address` is not supported for using as collateral
  /// Returns `InsufficientAllowanceForCollateral` if the caller does not have enough allowance
  /// Returns `InsufficientCollateralBalance` if the caller does not have enough balance
  /// Returns `AssetNotSupported` if the borrowing asset is not supported for borrowing
  /// Returns `AmountNotSupported` if the liquidation price is less than or equal to the borrowed amount
  /// Returns `InsufficientBalanceInContract` if there is not enough amount of assets in the contract to borrow
  #[ink(message)]
  fn borrow_assets(
    &mut self,
    asset_address: AccountId,
    collateral_address: AccountId,
    amount: Balance,
  ) -> Result<(), LendingError>;

  /// This function is called by the user who borrowed some asset. User needs to deposit borrowed amount along with interest
  /// They can either repay the full amount or just a portion of the amount. If they repay the full amount, they will get all deposited
  /// collateral back, another way they will get back the same portion of collateral as the repay portion (eg. if they deposit 80% of
  /// the loan + interests, they will get 80% of collateral back). If the loan was liquidated, the user does not get their collateral
  /// back and the NFT will be burned
  ///
  /// `loan_id` is the id of the loan to be repaid
  /// `repay_amount` is the amount of borrowed asset to be repaid
  ///
  /// Returns true if the loan was repaid successfuly, false if the loan was already liquidated and can not be repaid
  /// Returns `NotTheOwner` error if the initiator is not the owner of the loan token
  /// Returns `InsufficientAllowanceToRepay` error if the initiator did not give allowance to the contract
  /// Returns `InsufficientBalanceToRepay` error if the initiator tries to repay more tokens than their balance
  #[ink(message)]
  fn repay(&mut self, loan_id: Id, repay_amount: Balance) -> Result<bool, LendingError>;

  /// This function is called by the user who wants to withdraw assets they deposited for lending. They will deposit their
  /// share tokens and get back their share of the asset mapped to this share token
  ///
  /// `shares_address` account id of the shares token which is binded to the asset
  /// `shares_amount` amount of shares being withdrawn
  ///
  /// Returns `InsufficientBalanceInContract` if there is currently not enough assets in the contract
  #[ink(message)]
  fn withdraw_asset(&mut self, shares_address: AccountId, shares_amount: Balance) -> Result<(), LendingError>;

  /// This function will liquidate the loan with `loan_id`. In this example contract the tokens will be kept in the smart
  /// contract and the liquidator gets 1% of the liquidated assets. In a real implementation we would swap the collateral
  /// for the borrowed asset so we would be able to cover the shares of lenders.
  ///
  /// `loan_id` id of loan to be liquidated
  ///
  /// Returns `LoanLiquidated` error if the loan was already liquidated
  /// Returns `CanNotBeLiquidated` error if the price of collateral is not below the liquidation price
  #[ink(message)]
  fn liquidate_loan(&mut self, loan_id: Id) -> Result<(), LendingError>;
}

#[openbrush::wrapper]
pub type LendingPermissionedRef = dyn LendingPermissioned;

#[openbrush::trait_definition]
pub trait LendingPermissioned {
  /// This function will allow an asset to be accepted by the contract
  /// It will also create the contracts for the shares token and lended reserves token
  #[ink(message, payable)]
  fn allow_asset(&mut self, asset_address: AccountId) -> Result<(), LendingError>;

  /// This function will disallow lending and borrowing of asset
  /// To do this all assets of this asset must be repaid and all of the asset must be withdrawn
  #[ink(message)]
  fn disallow_lending(&mut self, asset_address: AccountId) -> Result<(), LendingError>;

  /// This function will allow an asset to be accepted as collateral
  #[ink(message)]
  fn allow_collateral(&mut self, asset_address: AccountId) -> Result<(), LendingError>;

  /// This function will disallow an asset to be accepted as collateral
  #[ink(message)]
  fn disallow_collateral(&mut self, asset_address: AccountId) -> Result<(), LendingError>;

  /// This function will set price of `asset_in` in `asset_out` to `amount` in our simulated oracle
  #[ink(message)]
  fn set_asset_price(
    &mut self,
    asset_in: AccountId,
    asset_out: AccountId,
    price: Balance,
  ) -> Result<(), LendingError>;
}
```

## Data and storage trait

In the `impls/lending/data.rs` we will define the data related to the lending contract.
After, we will define a storage trait via `declare_storage_trait!` macro to return a
`LendingData` and define some helper functions. 

In this example we will not be using price oracles, we will do
our own simulated oracle. Since oracles are not the point of this example,
it will be enough for us. We will store prices info in our data struct.

```rust
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
```

Derive macro for `LendingStorage` is created by the [description](/smart-contracts/example/data#macros-from-openbrush).

## A generic implementation of `LendingPermissioned` trait

The all methods in `LendingPermissioned` are restricted and requires `#[modifiers(only_role(MANAGER))]`.
That means that only accounts with `MANAGER` role can execute these methods.
Usage of `only_role` modifier from [access_control](https://github.com/Supercolony-net/openbrush-contracts/blob/main/contracts/src/access/access_control/mod.rs#L30)
requires that the contract should implement `AccessControlStorage`.
For that we also require the same restriction on the generic type.

In the implementation of `LendingPermissioned`, we want to use methods from
`Lending`. For that, the set of restrictions for generic in the `Lending` implementation
should be a subset(<=) of restrictions for generic in the `LendingPermissioned` implementation.
The `Lending` implementation requires `LendingStorage` and `PausableStorage` to use `when_paused` 
modifier from [pausable](https://github.com/Supercolony-net/openbrush-contracts/blob/main/contracts/src/security/pausable/mod.rs#L24).
So we should have the same restriction in our generic implementation.

In the logic of the trait `LendingPermissioned` we need to instantiate 
the `SharesContract`. But we can't import `SharesContract` into `lending_project` 
crate, because `SharesContract` also depends on `lending_project`. 
That will cause cyclic dependencies.
To avoid that we will import `SharesContract` into `LendingContract` and in `LendingContract` we will define
`_instantiate_shares_contract` method, that will instantiate `SharesCotnract`.
```rust
impl LendingPermissionedInternal for LendingContract {
    fn _instantiate_shares_contract(&self, contract_name: &str, contract_symbol: &str) -> AccountId {
    let code_hash = self.lending.shares_contract_code_hash;
    let (hash, _) =
        ink_env::random::<ink_env::DefaultEnvironment>(contract_name.as_bytes()).expect("Failed to get salt");
    let hash = hash.as_ref();
    let contract = SharesContractRef::new(Some(String::from(contract_name)), Some(String::from(contract_symbol)))
            .endowment(0)
            .code_hash(code_hash)
            .salt_bytes(&hash[..4])
            .instantiate()
            .unwrap();
    contract.to_account_id()
    }
}
```

For that we defined the `LendingPermissionedInternal` trait with `_instantiate_shares_contract` method.

The final generic implementation of the `LendingPermissioned` restricts the generic type `T`
by `LendingStorage`, `AccessControlStorage`, `PausableStorage`, `LendingPermissionedInternal` traits.
That allows us to use methods from these traits and define the implementation.
```rust
pub use super::data::*;
use openbrush::{
    contracts::{
        access_control::*,
        pausable::{
            PausableData,
            PausableStorage,
        },
        traits::psp22::PSP22Ref,
    },
    modifiers,
    traits::{
        AccountId,
        Balance,
        ZERO_ADDRESS,
    },
};

pub const MANAGER: RoleType = ink_lang::selector_id!("MANAGER");

impl<T> LendingPermissioned for T
where
    T: LendingStorage<Data = LendingData>
        + PausableStorage<Data = PausableData>
        + LendingPermissionedInternal
        + AccessControlStorage<Data = AccessControlData>,
{
    #[modifiers(only_role(MANAGER))]
    default fn allow_asset(&mut self, asset_address: AccountId) -> Result<(), LendingError> {
        // we will ensure the asset is not accepted already
        if self.is_accepted_lending(asset_address) {
            return Err(LendingError::AssetSupported)
        }

        // instantiate the shares of the lended assets
        let shares_address = self._instantiate_shares_contract("LendingShares", "LS");
        // instantiate the reserves of the borrowed assets
        let reserves_address = self._instantiate_shares_contract("LendingReserves", "LR");
        // accept the asset and map shares and reserves to it

        accept_lending(self, asset_address, shares_address, reserves_address);
        Ok(())
    }

    #[modifiers(only_role(MANAGER))]
    default fn disallow_lending(&mut self, asset_address: AccountId) -> Result<(), LendingError> {
        let reserve_asset = get_reserve_asset(self, &asset_address)?;
        if PSP22Ref::balance_of(&asset_address, Self::env().account_id()) > 0
            || PSP22Ref::balance_of(&reserve_asset, Self::env().account_id()) > 0
        {
            return Err(LendingError::AssetsInTheContract)
        }
        disallow_lending(self, asset_address);
        Ok(())
    }

    #[modifiers(only_role(MANAGER))]
    default fn allow_collateral(&mut self, asset_address: AccountId) -> Result<(), LendingError> {
        // we will ensure the asset is not accepted already
        if self.is_accepted_collateral(asset_address) {
            return Err(LendingError::AssetSupported)
        }
        set_collateral_accepted(self, asset_address, true);
        Ok(())
    }

    #[modifiers(only_role(MANAGER))]
    default fn disallow_collateral(&mut self, asset_address: AccountId) -> Result<(), LendingError> {
        // we will ensure the asset is not accepted already
        if self.is_accepted_collateral(asset_address) {
            set_collateral_accepted(self, asset_address, false);
        }
        Ok(())
    }

    #[modifiers(only_role(MANAGER))]
    default fn set_asset_price(
        &mut self,
        asset_in: AccountId,
        asset_out: AccountId,
        price: Balance,
    ) -> Result<(), LendingError> {
        set_asset_price(self, &asset_in, &asset_out, &price);
        Ok(())
    }
}

pub trait LendingPermissionedInternal {
    /// internal function which instantiates a shares contract and returns its AccountId
    fn _instantiate_shares_contract(&self, contract_name: &str, contract_symbol: &str) -> AccountId;
}

fn accept_lending<T: LendingStorage<Data = LendingData>>(
    instance: &mut T,
    asset_address: AccountId,
    share_address: AccountId,
    reserve_address: AccountId,
) {
    instance.get_mut().asset_shares.insert(&asset_address, &share_address);
    instance.get_mut().shares_asset.insert(&share_address, &asset_address);
    instance
        .get_mut()
        .assets_lended
        .insert(&asset_address, &reserve_address);
}

fn disallow_lending<T: LendingStorage<Data = LendingData>>(instance: &mut T, asset_address: AccountId) {
    let share_address = instance
        .get_mut()
        .asset_shares
        .get(&asset_address)
        .unwrap_or(ZERO_ADDRESS.into());
    instance.get_mut().asset_shares.remove(&asset_address);
    instance.get_mut().shares_asset.remove(&share_address);
    instance.get_mut().assets_lended.remove(&asset_address);
}

/// this function will accept `asset_address` for using as collateral
fn set_collateral_accepted<T: LendingStorage<Data = LendingData>>(
    instance: &mut T,
    asset_address: AccountId,
    accepted: bool,
) {
    instance.get_mut().collateral_accepted.insert(&asset_address, &accepted);
}
```

## A generic implementation of `Lending` trait

The same logic is used during definition of the implementation for `Lending` trait.

The `PausableStorage` restriction is required to use `when_paused`, `when_not_paused` modifiers 
from [pausable](https://github.com/Supercolony-net/openbrush-contracts/blob/main/contracts/src/security/pausable/mod.rs#L24).

```rust
// importing everything publicly from traits allows you to import every stuff related to lending
// by one import
pub use super::data::*;
pub use crate::traits::lending::*;
use crate::traits::{
    loan::{
        LoanInfo,
        LoanRef,
    },
    shares::SharesRef,
};
use ink_prelude::vec::Vec;
use openbrush::{
    contracts::{
        pausable::*,
        traits::{
            psp22::PSP22Ref,
            psp34::Id,
        },
    },
    modifiers,
    traits::{
        AccountId,
        AccountIdExt,
        Balance,
        Timestamp,
        ZERO_ADDRESS,
    },
};

pub const YEAR: Timestamp = 60 * 60 * 24 * 365;

impl<T: LendingStorage<Data = LendingData> + PausableStorage<Data = PausableData>> Lending for T {
    default fn total_asset(&self, asset_address: AccountId) -> Result<Balance, LendingError> {
        // get asset from mapping
        let mapped_asset = LendingStorage::get(self)
            .assets_lended
            .get(&asset_address)
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

    default fn total_shares(&self, asset_address: AccountId) -> Result<Balance, LendingError> {
        // get asset from mapping
        let mapped_asset = LendingStorage::get(self)
            .asset_shares
            .get(&asset_address)
            .unwrap_or(ZERO_ADDRESS.into());
        // return error if the asset is not supported
        if mapped_asset.is_zero() {
            return Err(LendingError::AssetNotSupported)
        }
        Ok(PSP22Ref::total_supply(&mapped_asset))
    }

    default fn is_accepted_lending(&self, asset_address: AccountId) -> bool {
        !LendingStorage::get(self)
            .asset_shares
            .get(&asset_address)
            .unwrap_or(ZERO_ADDRESS.into())
            .is_zero()
    }

    default fn is_accepted_collateral(&self, asset_address: AccountId) -> bool {
        LendingStorage::get(self)
            .collateral_accepted
            .get(&asset_address)
            .unwrap_or(false)
    }

    #[modifiers(when_not_paused)]
    default fn lend_assets(&mut self, asset_address: AccountId, amount: Balance) -> Result<(), LendingError> {
        // we will be using these often so we store them in variables
        let lender = Self::env().caller();
        let contract = Self::env().account_id();
        // ensure the user gave allowance to the contract
        if PSP22Ref::allowance(&asset_address, lender, contract) < amount {
            return Err(LendingError::InsufficientAllowanceToLend)
        }
        // ensure the user has enough assets
        if PSP22Ref::balance_of(&asset_address, lender) < amount {
            return Err(LendingError::InsufficientBalanceToLend)
        }
        // how much assets is already in the contract
        // if the asset is not accepted by the contract, this function will return an error
        let total_asset = self.total_asset(asset_address)?;
        // transfer the assets from user to the contract|
        PSP22Ref::transfer_from_builder(&asset_address, lender, contract, amount, Vec::<u8>::new())
            .call_flags(ink_env::CallFlags::default().set_allow_reentry(true))
            .fire()
            .unwrap()?;
        // if no assets were deposited yet we will mint the same amount of shares as deposited `amount`
        let new_shares = if total_asset == 0 {
            amount
        } else {
            // else we calculate how much shares will belong us after depositing the `amount`
            (amount * self.total_shares(asset_address)?) / total_asset
        };
        let reserve_asset = get_reserve_asset(self, &asset_address)?;
        // mint the shares token to the user
        SharesRef::mint(&reserve_asset, lender, new_shares)?;
        Ok(())
    }

    #[modifiers(when_not_paused)]
    default fn borrow_assets(
        &mut self,
        asset_address: AccountId,
        collateral_address: AccountId,
        amount: Balance,
    ) -> Result<(), LendingError> {
        // we will be using these often so we store them in variables
        let borrower = Self::env().caller();
        let contract = Self::env().account_id();
        // ensure this asset is accepted as collateral
        if !self.is_accepted_collateral(collateral_address) {
            return Err(LendingError::AssetNotSupported)
        }
        // ensure the user gave allowance to the contract
        if PSP22Ref::allowance(&collateral_address, borrower, contract) < amount {
            return Err(LendingError::InsufficientAllowanceForCollateral)
        }
        // ensure the user has enough collateral assets
        if PSP22Ref::balance_of(&collateral_address, borrower) < amount {
            return Err(LendingError::InsufficientCollateralBalance)
        }
        let reserve_asset = get_reserve_asset(self, &asset_address)?;

        // we will find out the price of deposited collateral
        let price = get_asset_price(self, &amount, &collateral_address, &asset_address);
        // we will set the liquidation price to be 75% of current price
        let liquidation_price = (price * 75) / 100;
        // borrow amount is 70% of collateral
        let borrow_amount = (price * 70) / 100;
        // ensure the liquidation price is greater than borrowed amount to avoid misuses
        if borrow_amount >= liquidation_price {
            return Err(LendingError::AmountNotSupported)
        }
        // ensure we have enough assets in the contract
        if PSP22Ref::balance_of(&asset_address, contract) < borrow_amount {
            return Err(LendingError::InsufficientBalanceInContract)
        }
        // we will transfer the collateral to the contract
        PSP22Ref::transfer_from_builder(&collateral_address, borrower, contract, amount, Vec::<u8>::new())
            .call_flags(ink_env::CallFlags::default().set_allow_reentry(true))
            .fire()
            .unwrap()?;
        // create loan info
        let loan_info = LoanInfo {
            borrower,
            collateral_token: collateral_address,
            collateral_amount: amount,
            borrow_token: asset_address,
            borrow_amount,
            liquidation_price,
            timestamp: Self::env().block_timestamp(),
            liquidated: false,
        };

        let load_account = LendingStorage::get(self).loan_account;
        LoanRef::create_loan(&load_account, loan_info)?;
        // transfer assets to borrower
        PSP22Ref::transfer(&asset_address, borrower, borrow_amount, Vec::<u8>::new())?;
        // mint `borrow_amount` of the reserve token
        SharesRef::mint(&reserve_asset, contract, borrow_amount)?;
        Ok(())
    }

    default fn repay(&mut self, loan_id: Id, repay_amount: Balance) -> Result<bool, LendingError> {
        // REPAYING (borrower: B, nft, repayAmount: X):
        let initiator = Self::env().caller();
        let contract = Self::env().account_id();
        let loan_account = LendingStorage::get(self).loan_account;
        let apy = 1000;
        // initiator must own the nft
        if LoanRef::owner_of(&loan_account, loan_id.clone()).unwrap_or(ZERO_ADDRESS.into()) != initiator {
            return Err(LendingError::NotTheOwner)
        }
        let loan_info = LoanRef::get_loan_info(&loan_account, loan_id.clone())?;
        if loan_info.liquidated {
            LoanRef::delete_loan(&loan_account, initiator, loan_id.clone())?;
            return Ok(false)
        }

        // ensure initiator has enough allowance
        if PSP22Ref::allowance(&loan_info.borrow_token, initiator, contract) < repay_amount {
            return Err(LendingError::InsufficientAllowanceToRepay)
        }
        // ensure initiator has enough balance
        if PSP22Ref::balance_of(&loan_info.borrow_token, initiator) < repay_amount {
            return Err(LendingError::InsufficientBalanceToRepay)
        }
        let time_passed = Self::env().block_timestamp() - loan_info.timestamp;
        let total_apy = (apy * time_passed as Balance) / YEAR as Balance;
        let to_repay = (((loan_info.borrow_amount) * (10000 + total_apy)) / 10000) + 1;
        let reserve_asset = get_reserve_asset(self, &loan_info.borrow_token)?;
        if repay_amount >= to_repay {
            PSP22Ref::transfer_from_builder(&loan_info.borrow_token, initiator, contract, to_repay, Vec::<u8>::new())
                .call_flags(ink_env::CallFlags::default().set_allow_reentry(true))
                .fire()
                .unwrap()?;
            PSP22Ref::transfer(
                &loan_info.collateral_token,
                initiator,
                loan_info.collateral_amount,
                Vec::<u8>::new(),
            )?;
            LoanRef::delete_loan(&loan_account, initiator, loan_id)?;
            SharesRef::burn(&reserve_asset, Self::env().caller(), loan_info.borrow_amount)?;
        } else {
            PSP22Ref::transfer_from_builder(
                &loan_info.borrow_token,
                initiator,
                contract,
                repay_amount,
                Vec::<u8>::new(),
            )
            .call_flags(ink_env::CallFlags::default().set_allow_reentry(true))
            .fire()
            .unwrap()?;
            let to_return = (repay_amount * loan_info.collateral_amount) / to_repay;
            PSP22Ref::transfer(&loan_info.collateral_token, initiator, to_return, Vec::<u8>::new())?;
            SharesRef::mint(
                &reserve_asset,
                contract,
                to_repay - repay_amount - loan_info.borrow_amount,
            )?;
            LoanRef::update_loan(
                &loan_account,
                loan_id.clone(),
                to_repay - repay_amount,
                Self::env().block_timestamp(),
                loan_info.collateral_amount - to_return,
            )?;
        }
        Ok(true)
    }

    default fn withdraw_asset(
        &mut self,
        shares_address: AccountId,
        shares_amount: Balance,
    ) -> Result<(), LendingError> {
        let withdraw_asset = get_asset_from_shares(self, &shares_address)?;
        let withdraw_amount =
            (shares_amount * self.total_asset(withdraw_asset)?) / PSP22Ref::total_supply(&shares_address);
        if withdraw_amount > PSP22Ref::balance_of(&withdraw_asset, Self::env().account_id()) {
            return Err(LendingError::InsufficientBalanceInContract)
        }

        SharesRef::burn(&shares_address, Self::env().caller(), shares_amount)?;
        PSP22Ref::transfer(&withdraw_asset, Self::env().caller(), withdraw_amount, Vec::<u8>::new())?;
        Ok(())
    }

    default fn liquidate_loan(&mut self, loan_id: Id) -> Result<(), LendingError> {
        let loan_account = LendingStorage::get(self).loan_account;
        let loan_info = LoanRef::get_loan_info(&loan_account, loan_id.clone())?;

        if loan_info.liquidated {
            return Err(LendingError::LoanLiquidated)
        }

        let price = get_asset_price(
            self,
            &loan_info.collateral_amount,
            &loan_info.collateral_token,
            &loan_info.borrow_token,
        );

        if price <= loan_info.liquidation_price {
            // if we swapped the collateral to borrow asset we would burn the reserve tokens
            // let reserve_asset = self._get_reserve_asset(borrow_asset);
            // PSP22BurnableRef::burn(&reserve_asset, borrow_amount)
            let reward = (loan_info.collateral_amount * 1000) / 100000;
            PSP22Ref::transfer(
                &loan_info.collateral_token,
                Self::env().caller(),
                reward,
                Vec::<u8>::new(),
            )?;
            LoanRef::liquidate_loan(&loan_account, loan_id.clone())?;
        } else {
            return Err(LendingError::CanNotBeLiquidated)
        }
        Ok(())
    }
}

```