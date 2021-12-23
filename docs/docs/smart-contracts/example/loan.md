---
sidebar_position: 5
title: Loan token
---

In our project we will be also implementing [PSP-721](/smart-contracts/PSP721/psp721) token. This token will represent a loan of a user who borrowed some assets. Upon borrowing assets the contract will mint an NFT to them, which will hold the information about their loan, namely the user who borrowed the assets, address of the asset which was used as collateral, how much collateral was deposited, what asset was borrowed, and how much, the liquidation price of the loan, timestamp of when was the loan performed, and information whether the loan is liquidated or not. This data will be stored in a separate storage trait, which we will derive in our NFT contract. We do this to separate storage from the logic, and we will do this in the lending contract as well. We do not want anybody to just mint and burn these, so we will implement the [Ownable](/smart-contracts/ownable) extension in our NFT. The mint and burn logic will be covered differently, we will not be using the mintable and burnable extensions.

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
brush_derive = { path = "../../../../utils/brush_derive" }

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

use brush_derive::declare_derive_storage_trait;

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
use ink_prelude::{
    string::String,
    vec::Vec,
};
use ink_storage::{
    collections::HashMap as StorageHashMap,
    traits::SpreadLayout,
};
pub use loan_derive::LoanStorage;
use psp721::traits::*;
```

And then we will define the storage struct. `last_loan_id` will be the id of last loan token, so we can mantain unique ids in each new NFT, and `freed_ids` will contain ids of deleted NFTs, which will be reused in new loans.

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
    last_loan_id: Id,
    freed_ids: Vec<Id>,
}

declare_storage_trait!(LoanStorage, LoanData);
```

We will also declare a type `LoanInfo`, which will be a tuple of the loan data, which we will use on other places.

```rust
pub type LoanInfo = (
    AccountId,
    AccountId,
    Balance,
    AccountId,
    Balance,
    Balance,
    Timestamp,
    bool,
);
```

And finally, add some functions for our trait. In order to call our NFT contract's functions in other contracts we will create a wrapper around this trait using `brush::wrapper` attribute, inherit this trait in our NFT contract and implement the unimplemented functions to work with our contract. You can check the logic of each function in the example, here we will just show the headers of the functions. We will declare the trait like this:

```rust
#[brush::wrapper]
pub type LoanRef = dyn LoanTrait;

#[brush::trait_definition]
pub trait LoanTrait: LoanStorage {
    #[ink(message)]
    fn create_loan(
        &mut self,
        borrower: AccountId,
        collateral_asset: AccountId,
        collateral_amount: Balance,
        borrow_asset: AccountId,
        borrow_amount: Balance,
        liquidation_price: Balance,
        timestamp: Timestamp,
    ) -> Result<(), PSP721Error>;

    fn delete_loan(&mut self, initiator: AccountId, loan_id: Id) -> Result<(), PSP721Error>;

    #[ink(message)]
    fn update_loan(
        &mut self,
        loan_id: Id,
        new_borrow_amount: Balance,
        new_timestamp: Timestamp,
        new_collateral_amount: Balance,
    ) -> Result<(), PSP721Error>;

    #[ink(message)]
    fn liquidate_loan(&mut self, loan_id: Id) -> Result<(), PSP721Error>;

    #[ink(message)]
    fn get_loan_info(&self, loan_id: Id) -> Result<LoanInfo, PSP721Error> {
        todo()!;
    }

    fn _init(&mut self) {
        todo()!;
    }

    fn _init_loan(
        &mut self,
        borrower: AccountId,
        collateral_asset: AccountId,
        collateral_amount: Balance,
        borrow_asset: AccountId,
        borrow_amount: Balance,
        liquidation_price: Balance,
        timestamp: Timestamp,
    ) -> Result<Id, PSP721Error> {
        todo()!;
    }

    fn _delete_loan(&mut self, loan_id: Id) {
        todo()!;
    }

    fn _update_loan(
        &mut self,
        loan_id: Id,
        new_borrow_amount: Balance,
        new_timestamp: Timestamp,
        new_collateral_amount: Balance,
    ) -> Result<(), PSP721Error> {
        todo()!;
    }

    fn _liquidate_loan(&mut self, loan_id: Id) -> Result<(), PSP721Error> {
        todo()!;
    }

    fn _get_next_loan_id_and_increase(&mut self) -> Result<Id, PSP721Error> {
        todo()!;
    }
}
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

Along with the ink dependencies.

## Implement the contract

We want a basic [PSP-721](/smart-contracts/PSP721/psp721) token with metadata and ownable extensions, so we will add these to our contract. We will add a `brush::contract` macro to our contract and add some imports:

```rust
#[brush::contract]
pub mod loan {
    use crate::traits::*;
    pub use crate::traits::{
        LoanInfo,
        LoanRef,
    };
    use brush::modifiers;
    use ink_lang::{
        EmitEvent,
        Env,
    };
    use ink_prelude::string::String;
    use ownable::traits::*;
    use psp721::{
        extensions::metadata::*,
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

```

## Implement the LoanTrait trait

We will implement the storage trait, but in this trait we have declared some functions without a body, so we need to define the body in this `impl` block. We also want these functions to only be callable by the owner of the contract, so we will add to them the `only_owner` modifier.

```rust
impl LoanTrait for Loan {
    #[modifiers(only_owner)]
    #[ink(message)]
    fn create_loan(
        &mut self,
        borrower: AccountId,
        collateral_asset: AccountId,
        collateral_amount: Balance,
        borrow_asset: AccountId,
        borrow_amount: Balance,
        liquidation_price: Balance,
        timestamp: Timestamp,
    ) -> Result<(), PSP721Error> {
        todo()!;
    }

    #[modifiers(only_owner)]
    #[ink(message)]
    fn delete_loan(&mut self, initiator: AccountId, loan_id: Id) -> Result<(), PSP721Error> {
        todo()!;
    }

    #[modifiers(only_owner)]
    #[ink(message)]
    fn update_loan(
        &mut self,
        loan_id: Id,
        new_borrow_amount: Balance,
        new_timestamp: Timestamp,
        new_collateral_amount: Balance,
    ) -> Result<(), PSP721Error> {
        todo()!;
    }

    #[modifiers(only_owner)]
    #[ink(message)]
    fn liquidate_loan(&mut self, loan_id: Id) -> Result<(), PSP721Error> {
        todo()!;
    }
}
```

The fact that we created a wrapper over our `LoanTrait` allows us to call for example the `create_loan` function from other contracts with `LoanRef::(&contract_address, ..args)`, where `contract_address` is the account id of contract on which we want to the call the function and `args` are the arguments needed to call that function.

## Define the constructor and add functions

Finally, we will define the constructor where we will set the name and the symbol of the token and then initialize the owner of the token (which then will be able to mint and burn the tokens). We will also add a function for creating loans, which will mint a new loan token and initialize its data to storage. This function will be again restricted only to the owner.

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