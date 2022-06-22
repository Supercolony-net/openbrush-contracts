---
sidebar_position: 10
title: Notes about methods
---

In this section, we describe the implementation of the functions of our lending 
contract.

## Instantiating contracts

Each asset that we will accept to be lent will have two underlying tokens: 
the shares token and the reserves token. The shares token will represent a 
user's share of the lent asset which they can then withdraw and the reserves 
token will represent the amount of asset lent since we don't want to keep 
track of all addresses and amounts which have borrowed the assets. We will 
simply take this amount from the total supply of the underlying reserve token. 
So when we are accepting an asset for lending, we need to create a new token 
contract for shares and for reserves. We will define an internal function for 
this:

```rust
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
```

This function will instantiate our `SharesContract` contract and return 
the `AccountId` of the instantiated contract. We will call this function 
when allowing assets.

## Simulating oracle

As mentioned before, we will not be using a price oracle in our example, 
but we will use our own simulated oracle. And by simulated we mean adding 
some storage fields which hold the info about price of an asset and a function 
only callable by the account with `MANAGER` role, which will set the price of 
the asset. For that we define these functions:

```rust
#[modifiers(only_role(MANAGER))]
default fn set_asset_price(
    &mut self,
    asset_in: AccountId,
    asset_out: AccountId,
    price: Balance,
) -> Result<(), LendingError> {
    set_asset_price(self, asset_in, asset_out, price);
    Ok(())
}

/// this internal function will be used to set price of `asset_in` when we deposit `asset_out`
/// we are using this function in our example to simulate an oracle
pub fn set_asset_price<T: LendingStorage>(instance: &mut T, asset_in: AccountId, asset_out: AccountId, price: Balance) {
    instance.get_mut().asset_price.insert((&asset_in, &asset_out), &price);
}
```

## Allowing assets

If we just started lending and borrowing random assets or using random assets 
as collateral there would be chaos in our smart contract. 
Regarding lending, it would not be a big problem, since if somebody is 
willing to borrow an asset, it would generate a profit for the lender. 
But if we started accepting random assets as collateral, anyone could just 
throw a random coin as collateral and then just for example rug pull it and 
also keep the borrowed assets. Because of this we will only accept certain 
assets for lending and using as collateral. For an asset to be accepted, an 
account with the `MANAGER` role needs to allow it with the `allow_asset` function. 
We will use a modifier from OpenBrush, which serves similarly to Solidity's 
function modifiers. The function will look like this:

```rust
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
```

## Lending assets

For lending the assets  we will use the function `lend_assets(asset_address, amount)`, 
where `asset_address` is the address of `PSP-22` we want to deposit and `amount` 
is the amount of asset deposited. Some checks need to be checked to assure the correct 
behavior of our contract. The asset deposited needs to be recognized by our contract 
(manager must have approved it). If it is not accepted, an error will be returned. 
Then the user must have approved the asset to spent by our contract and the user's 
balance must be greater than or equal to `amount`. So we will transfer the asset from 
the user to the contract, mint shares to the user. To perform a cross contract call 
we will be using the references to contracts `SharesRef`. 
We will also add `when_not_paused` modifier to this function, 
so it can be only called when the contract is not paused. 
The code will look like this:

```rust

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
```

## Borrowing assets

The `borrow_assets(asset_address, collateral_address, amount)` function will 
serve for the users to borrow assets from the smart contract. 
`asset_address` is the account id of the asset we want to borrow, 
`collateral_address` is the account id of asset which the user wants 
to use as collateral, and `amount` is the amount of collateral deposited. 
Our contract will calculate the value of the deposited collateral and 
will give the borrower 70% of the collateral value. For pricing, we would 
use an oracle, but in this example, we will use our 'simulated oracle' - 
we will just store the price info in our contract and the admin will 
be able to change it. The liquidation price of the loan will be calculated 
at 75% of the collateral value. First of all the contract must not be paused, 
for which we use modifier `when_not_paused`. After that, for the borrowing 
to succeed, the `collateral_address` must be accepted by the contract, 
the contract needs to have enough allowance to spend the borrower's collateral 
token, borrower's collateral balance must be equal to or greater than `amount` 
and finally, the `asset_address` must be accepted for borrowing in the 
smart contract. After we calculate the liquidation price and borrow amount, 
we ensure the contract has enough assets to provide for the borrower, 
and we also want the liquidation price of the collateral to be higher than 
the borrowed amount. Since we are dealing with integers, entering a very 
low amount (below 10) of collateral may result in the liquidation price being 
the same as the borrowed amount, which could be exploited. We can surely 
handle it in many different ways, but again, it is not the purpose of this 
example so we will deal with it this way. When everything is alright, we will 
transfer the collateral to the contract, mint an NFT, which stores the 
information about the loan, to the borrower, then transfer the asset to the 
borrower, and finally, mint the reserve token. We will mint the same amount 
that we lent, and we will burn it after the loan is repaid or liquidated. 
This reserve token will be used to track the amount of the asset which is 
currently borrowed.

```rust
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
```