---
sidebar_position: 9
title: Implementation
---

In this section we will implement the functions of our lending contract.

## Emitting events

We have defined our events in the previous step, now we will define helper functions, with which we will be emitting our events. We will start all our helper functions with `_`.

```rust
fn _emit_lending_accepted_event(
    &self,
    asset_address: AccountId,
    shares_address: AccountId,
    reserves_address: AccountId,
    manager_address: AccountId,
) {
    self.env().emit_event(LendingAllowed {
        asset_address,
        shares_address,
        reserves_address,
        manager_address,
    });
}

fn _emit_lend_event(&self, lender: AccountId, asset: AccountId, amount: Balance) {
    self.env().emit_event(Lend { lender, asset, amount });
}
```

## Instantiating contracts

Each asset that we will accept to be lent will have two underlying tokens: the shares token and the reserves token. The shares token will represent a user's share of the lent asset which they can then withdraw and the reserves token will represent the amount of asset lent since we don't want to keep track of all addresses and amounts which have borrowed the assets. We will simply take this amount from the total supply of the underlying reserve token. So when we are accepting an asset for lending, we need to create a new token contract for shares and for reserves. We will define an internal function for this:

```rust
fn _instantiate_shares_contract(&self, contract_name: &str, contract_symbol: &str) -> AccountId {
    let code_hash = self.code_hash;
    let contract = Shares::new(Some(String::from(contract_name)), Some(String::from(contract_symbol)))
        .endowment(25)
        .code_hash(code_hash)
        .salt_bytes(&[0xDE, 0xAD, 0xBE, 0xEF])
        .instantiate()
        .unwrap();
    contract.to_account_id()
}
```

This function will instantiate our `Shares` contract and return the `AccountId` of the instantiated contract. We will call this function when allowing assets.

## Allowing assets

If we just started lending and borrowing random assets or using random assets as collateral there would be chaos in our smart contract. Regarding lending, it would not be a big problem, since if somebody is willing to borrow an asset, it would generate a profit for the lender. But if we started accepting random assets as collateral, anyone could just throw a random coin as collateral and then just for example rug pull it and also keep the borrowed assets. Because of this we will only accept certain assets for lending and using as collateral. For an asset to be accepted, an account with the `MANAGER` role needs to allow it with the `allow_asset` function. We will use a modifier from OpenBrush, which serves similarly to Solidity's function modifiers. The function will look like this:

```rust
#[modifiers(only_role(MANAGER))]
#[ink(message)]
pub fn allow_asset(&mut self, asset_address: AccountId) -> Result<(), LendingError> {
    // we will ensure the asset is not accepted already
    if self.is_accepted_lending(asset_address) {
        return Err(LendingError::AssetSupported)
    }
    // instantiate the shares of the lended assets
    let shares_address = self._instantiate_shares_contract("LendingShares", "LS");
    // instantiate the reserves of the borrowed assets
    let reserves_address = self._instantiate_shares_contract("LendingReserves", "LR");
    // accept the asset and map shares and reserves to it
    self._accept_lending(asset_address, shares_address, reserves_address);
    Ok(())
}
```

## Lending assets

For lending the assets  we will use the function `lend_assets(asset_address, amount)`, where `asset_address` is the address of `PSP-22` we want to deposit and `amount` is the amount of asset deposited. Some checks need to be checked to assure the correct behavior of our contract. The asset deposited needs to be recognized by our contract (manager must have approved it). If it is not accepted, an error will be returned. Then the user must have approved the asset to spent by our contract and the user's balance must be greater than or equal to `amount`. So we will transfer the asset from the user to the contract, mint shares to the user and emit the `Lend` event. To perform a cross contract call we will be using the wrappers (`PSP22Wrapper`, `PSP22MintableWrapper`). The code will look like this:

```rust
#[ink(message)]
pub fn lend_assets(&mut self, asset_address: AccountId, amount: Balance) -> Result<(), LendingError> {
    // we will be using these often so we store them in variables
    let lender = Self::env().caller();
    let contract = Self::env().account_id();
    // ensure the user gave allowance to the contract
    if PSP22Wrapper::allowance(&asset_address, lender, contract) < amount {
        return Err(LendingError::InsufficientAllowanceToLend)
    }
    // ensure the user has enough assets
    if PSP22Wrapper::balance_of(&asset_address, lender) < amount {
        return Err(LendingError::InsufficientBalanceToLend)
    }
    // how much assets is already in the contract
    // if the asset is not accepted by the contract, this function will return an error
    let total_asset = self.total_asset(asset_address)?;
    // transfer the assets from user to the contract
    PSP22Wrapper::transfer_from(&asset_address, lender, contract, amount, Vec::<u8>::new())?;
    // if no assets were deposited yet we will mint the same amount of shares as deposited `amount`
    let new_shares = if total_asset == 0 {
        amount
    } else {
        // else we calculate how much shares will belong us after depositing the `amount`
        (amount * self.total_shares(asset_address)?) / total_asset
    };
    // mint the shares token to the user
    PSP22MintableWrapper::mint(&asset_address, lender, new_shares)?;
    // emit the lend event
    self._emit_lend_event(lender, asset_address, amount);
    Ok(())
}
```
