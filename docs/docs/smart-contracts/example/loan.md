---
sidebar_position: 5
title: Loan token
---

In our project we will be also implementing [PSP-721](/smart-contracts/PSP721/psp721) token. This token will represent a loan of a user who borrowed some assets. Upon borrowing assets the contract will mint an NFT to them, which will hold the information about their loan, namely the user who borrowed the assets, address of the asset which was used as collateral, how much collateral was deposited, what asset was borrowed, and how much, the liquidation price of the loan, timestamp of when was the loan performed, and information whether the loan is liquidated or not. This data will be stored in a separate storage trait, which we will derive in our NFT contract. We do this to separate storage from the logic, and we will do this in the lending contract as well. We do not want anybody to just mint and burn these, so we will implement the [Ownable](/smart-contracts/ownable) extension in our NFT, and also we want to burn and mint these, so we will implement [PSP-721 Burnable](/smart-contracts/PSP721/extensions/burnable) and [PSP-721 Mintable](/smart-contracts/PSP721/extensions/mintable) as well.

## File structure

For our NFT contract we need a `Cargo.toml` file, `lib.rs` file with the contract's logic, and `traits.rs` file, where we will declare our storage trait. Apart from that, we will create a `derive` folder, where we define our derive macro for deriving the storage trait. There we need `Cargo.toml` where we import dependencies:

```toml
[package]
name = "loan_derive"
version = "1.0.0"
authors = ["Supercolony <dominik.krizo@supercolony.net>"]
edition = "2018"

[dependencies]
syn = { version = "1.0" }
quote = "1.0"
proc-macro2 = "1"
brush_derive_macro = { path = "../../../../utils/brush_derive_macro" }

[lib]
name = "loan_derive"
path = "lib.rs"
proc-macro = true

[features]
default = ["std"]
std = []
```

And a `lib.rs` file, where we declare the derive macro, and we will do it like this:

```rust
#![cfg_attr(not(feature = "std"), no_std)]

extern crate proc_macro;

use brush_derive_macro::declare_derive_storage_trait;

declare_derive_storage_trait!(derive_loan_storage, LoanStorage, LoanStorageField);
```

## Storage trait

All the data about our loan will be stored in the storage trait, which we will derive in our NFT contract. The storage trait will be declared in the `traits.rs` file. First, we import stuff that we will be using in the trait: 

```rust
use brush::{
    declare_storage_trait,
    traits::{
        AccountId,
        Balance,
        InkStorage,
        Timestamp,
    },
};
use ink_storage::{
    collections::HashMap as StorageHashMap,
    traits::SpreadLayout,
};
pub use loan_derive::LoanStorage;
use psp721::traits::Id;
```

And then we will define the storage struct:

```rust
#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct LoanData {
    pub borrower: StorageHashMap<Id, AccountId>,
    pub collateral_asset: StorageHashMap<Id, AccountId>,
    pub collateral_amount: StorageHashMap<Id, Balance>,
    pub borrow_asset: StorageHashMap<Id, AccountId>,
    pub borrow_amount: StorageHashMap<Id, Balance>,
    pub liquidation_price: StorageHashMap<Id, Balance>,
    pub timestamp: StorageHashMap<Id, Timestamp>,
    pub liquidated: StorageHashMap<Id, bool>,
}

declare_storage_trait!(LoanStorage, LoanData);
```

And finally, add some functions for our trait:

```rust
#[brush::trait_definition]
pub trait LoanTrait: LoanStorage {}
```

## Add dependencies

In addition to the dependencies imported in the [PSP-721](/smart-contracts/PSP721/psp721) documentation, we will also add the `ownable` dependency, and a dependency on our derive file. We will be using this contract as a dependency in our lending contract, so we need to also add the `"rlib"` crate type. So in final, we will import these dependencies:

```toml
[dependencies]
psp721 = { path = "../../../contracts/token/psp721", default-features = false }
ownable = { path = "../../../contracts/access/ownable", default-features = false }
brush = { path = "../../../utils/brush", default-features = false }
loan_derive = { path = "derive" }

[lib]
name = "loan_nft"
path = "lib.rs"
crate-type = [
    "cdylib",
    "rlib"
]

[features]
default = ["std"]
std = [
    "ownable/std",
    "psp721/std",
    "brush/std",
]
```

along with ink dependencies.

## Implement the contract

We want a basic [PSP-721](/smart-contracts/PSP721/psp721) token with metadata, ownable, burnable and mintable extensions, so we will add these to our contract. We will add a `brush::contract` macro to our contract and add some imports:

```rust
#[brush::contract]
pub mod loan {
    use crate::traits::*;
    use brush::modifiers;
    use ink_lang::{
        EmitEvent,
        Env,
    };
    use ink_prelude::string::String;
    use ownable::traits::*;
    use psp721::{
        extensions::{
            burnable::*,
            metadata::*,
            mintable::*,
        },
        traits::*,
    };
```

## Define the storage

In this storage, we will also derive the storage traits related to `Ownable` and `Loan` and declare the fields related to these traits.

```rust
#[ink(storage)]
#[derive(Default, PSP721Storage, OwnableStorage, PSP721MetadataStorage, LoanStorage)]
pub struct Loan {
    #[PSP721StorageField]
    psp721: PSP721Data,
    #[OwnableStorageField]
    ownable: OwnableData,
    #[PSP721MetadataStorageField]
    metadata: PSP721MetadataData,
    #[LoanStorageField]
    loan: LoanData,
}
```

## Define Event structs

Our token will be emitting events on transfer and approval, so to do that, we need to define them and override the functions to emit them. We will define an event for `Approval`, `ApprovalForAll` and an event for `Transfer`.

```rust
#[ink(event)]
pub struct Transfer {
    #[ink(topic)]
    from: Option<AccountId>,
    #[ink(topic)]
    to: Option<AccountId>,
    #[ink(topic)]
    id: Id,
}

#[ink(event)]
pub struct Approval {
    #[ink(topic)]
    from: AccountId,
    #[ink(topic)]
    to: AccountId,
    #[ink(topic)]
    id: Id,
}

#[ink(event)]
pub struct ApprovalForAll {
    #[ink(topic)]
    owner: AccountId,
    #[ink(topic)]
    operator: AccountId,
    approved: bool,
}
```

We will also override the `_emit_transfer_event`, `_emit_approval_event` and `_emit_approval_for_all_event` functions and emit the events inside them.

```rust
impl PSP721 for Loan {
    fn _emit_transfer_event(&self, from: Option<AccountId>, to: Option<AccountId>, id: Id) {
        self.env().emit_event(Transfer { from, to, id });
    }

    fn _emit_approval_event(&self, from: AccountId, to: AccountId, id: Id) {
        self.env().emit_event(Approval { from, to, id });
    }

    fn _emit_approval_for_all_event(&self, owner: AccountId, operator: AccountId, approved: bool) {
        self.env().emit_event(ApprovalForAll {
            owner,
            operator,
            approved,
        });
    }
}
```

## Implement the extension traits

We will be using these extensions in our token, so we will implement them for our storage.

```rust
impl Ownable for Loan {}

impl PSP721Metadata for Loan {}

impl LoanTrait for Loan {}
```

## Implement the Burnable and Mintable traits

Now we will implement the `PSP721Burnable` and `PSP721Mintable` traits. Since we don't want anybody to mint or burn the tokens, we only want the owner, in this case, our lending contract, to do it. So we will add the `PSP721Burnable` and `PSP721Mintable` and mark the functions of these traits with the `only_owner` restriction.

```rust
impl PSP721Mintable for Loan {
    #[modifiers(only_owner)]
    #[ink(message)]
    fn mint(&mut self, id: Id) -> Result<(), PSP721Error> {
        self._mint(id)
    }

    #[modifiers(only_owner)]
    #[ink(message)]
    fn mint_to(&mut self, account: AccountId, id: Id) -> Result<(), PSP721Error> {
        self._mint_to(account, id)
    }
}

impl PSP721Burnable for Loan {
    #[modifiers(only_owner)]
    #[ink(message)]
    fn burn(&mut self, id: Id) -> Result<(), PSP721Error> {
        self._burn(id)
    }

    #[modifiers(only_owner)]
    #[ink(message)]
    fn burn_from(&mut self, account: AccountId, id: Id) -> Result<(), PSP721Error> {
        self._burn_from(account, id)
    }
}
```

This will restrict accounts other than the owner of the token (which will be the lending contract) from calling these functions.

## Define the constructor

Finally, we will define the constructor where we will set the name and the symbol of the token and then initialize the owner of the token (which then will be able to mint and burn the tokens.)

```rust
impl Loan {
    #[ink(constructor)]
    pub fn new() -> Self {
        let mut instance = Self::default();
        instance._init_with_metadata(Some(String::from("Loan NFT")), Some(String::from("L-NFT")));
        instance._init_with_owner(Self::env().caller());
        instance
    }
}
```