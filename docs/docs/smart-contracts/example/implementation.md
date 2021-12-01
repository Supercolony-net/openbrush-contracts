---
sidebar_position: 9
title: Implementation
---

In this section we will implement the functions of our lending contract.

## Emitting events

We have defined our events in the previous step, now we will define helper functions, with which we will be emitting our events. We will start all our helper functions with `_`.

```rust
#[brush::contract]
fn _emit_lend_event(&self, lender: AccountId, asset: AccountId, amount: Balance) {
    self.env().emit_event(Lend { lender, asset, amount });
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
