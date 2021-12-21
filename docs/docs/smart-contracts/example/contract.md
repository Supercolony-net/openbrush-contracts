---
sidebar_position: 9
title: Lending contract
---

Now we will define the contract's logic in our `lib.rs` file.

## Add imports and enable unstable feature

As everywhere, we will import all we need for our smart contract to work.

```rust
#[brush::contract]
pub mod lending {
    use crate::{
        errors::*,
        traits::*,
    };
    use access_control::traits::*;
    use brush::{
        modifiers,
        traits::{
            AccountIdExt,
            ZERO_ADDRESS,
        },
    };
    use ink_lang::ToAccountId;
    use ink_prelude::{
        string::String,
        vec::Vec,
    };
    use loan_nft::loan::{
        Loan,
        LoanRef,
    };
    use pausable::traits::*;
    use psp22::{
        extensions::{
            burnable::*,
            mintable::*,
        },
        traits::*,
    };
    use psp721::traits::Id;
    use shares::shares::Shares;
```

## Define the events

We will be keeping the track of events that happened in our smart contract. For that, we need to define the event structs and we will be emitting them when needed. We will be emitting the `Lend` event when assets are deposited and the `LendingAllowed` event when an asset is allowed for lending by a manager account.

```rust
#[ink(event)]
pub struct Lend {
    #[ink(topic)]
    lender: AccountId,
    #[ink(topic)]
    asset: AccountId,
    amount: Balance,
}

#[ink(event)]
pub struct LendingAllowed {
    #[ink(topic)]
    asset_address: AccountId,
    #[ink(topic)]
    shares_address: AccountId,
    #[ink(topic)]
    reserves_address: AccountId,
    #[ink(topic)]
    manager_address: AccountId,
}

#[ink(event)]
pub struct LendingCancelled {
    #[ink(topic)]
    asset_address: AccountId,
    #[ink(topic)]
    manager_address: AccountId,
}

#[ink(event)]
pub struct CollateralAllowed {
    #[ink(topic)]
    asset_address: AccountId,
    #[ink(topic)]
    manager_address: AccountId,
}

#[ink(event)]
pub struct CollateralCancelled {
    #[ink(topic)]
    asset_address: AccountId,
    #[ink(topic)]
    manager_address: AccountId,
}

#[ink(event)]
pub struct Borrow {
    #[ink(topic)]
    borrower: AccountId,
    #[ink(topic)]
    collateral_address: AccountId,
    #[ink(topic)]
    asset_address: AccountId,
    collateral_amount: Balance,
    borrow_amount: Balance,
}

#[ink(event)]
pub struct Repay {
    #[ink(topic)]
    borrower: AccountId,
    #[ink(topic)]
    collateral_address: AccountId,
    #[ink(topic)]
    asset_address: AccountId,
    collateral_amount: Balance,
    repay_amount: Balance,
    to_repay: Balance,
}

#[ink(event)]
pub struct Withdraw {
    #[ink(topic)]
    lender: AccountId,
    #[ink(topic)]
    asset_address: AccountId,
    withdraw_amount: Balance,
}

#[ink(event)]
pub struct Liquidate {
    #[ink(topic)]
    liquidator: AccountId,
    #[ink(topic)]
    borrower: AccountId,
    #[ink(topic)]
    collateral_address: AccountId,
    collateral_amount: Balance,
    liquidator_fee: Balance,
}
```

## Define the contract storage

As described earlier, we want our smart contract to be paused by the Manager accounts. To do that, we need our contract to be `Pausable` and we need a manager role. We can do this with the `AccessControl`. Also, we want to use the `LendingStorageTrait` we have declared. So we will declare a struct and derive all these traits needed. We will also store the `code_hash` of our `Shares` contract, because we will be instantiating it later, for which we need the hash, and we will also need the AccountId of our nft contract.

```rust
#[ink(storage)]
#[derive(Default, AccessControlStorage, PausableStorage, LendingStorage)]
pub struct Lending {
    #[AccessControlStorageField]
    access: AccessControlData,
    #[PausableStorageField]
    pause: PausableData,
    #[LendingStorageField]
    lending: LendingData,
    code_hash: Hash,
    nft_contract: AccountId,
}
```

## Define roles

What about the manager role we have mentioned? We will declare a `const RoleType` Manager and we will allow the admin to grant a Manager role to other accounts.

```rust
const MANAGER: RoleType = ink_lang::selector_id!("MANAGER");
```

## Implement traits

All the traits we need and we have declared storage fields for, we need to implement them so we can call the functions on our smart contract.

```rust
impl LendingStorageTrait for Lending {}

impl AccessControl for Lending {}

impl Pausable for Lending {}
```

## Define the constructor

Finally, we will add a constructor, in which we will initiate the admin of the contract, to whom we will also grant the manager role declared before, and we will also instantiate the NFT contract here and store its AccountId in our contract.

```rust
impl Lending {
    #[ink(constructor)]
    pub fn new(code_hash: Hash) -> Self {
        let mut instance = Self::default();
        let caller = instance.env().caller();
        instance._init_with_admin(caller);
        instance.grant_role(MANAGER, caller).expect("Can not set manager role");
        instance.code_hash = code_hash;
        let nft = Loan::new()
                .endowment(25)
                .code_hash(nft_code_hash)
                .salt_bytes(&[0xDE, 0xAD, 0xBE, 0xEF])
                .instantiate()
                .unwrap();
        instance.nft_contract = nft.to_account_id();
        instance
    }
}
```

We will take a look at the specific implementation of the functions of the smart contract now.