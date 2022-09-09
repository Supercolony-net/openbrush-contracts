---
sidebar_position: 5
title: Loan contract
---

In our project we will also implement [PSP-34](/smart-contracts/PSP34) 
token. This token will represent a loan of a user who borrowed some assets. 
Upon borrowing assets the contract will mint an NFT to them, which will hold 
the information about their loan, namely the user who borrowed the assets, 
address of the asset which was used as collateral, how much collateral was 
deposited, what asset was borrowed, and how much, the liquidation price of 
the loan, timestamp of when was the loan performed, and information whether 
the loan is liquidated or not. This data will be stored in a separate storage 
trait, which we will derive in our NFT contract. We do this to separate storage 
from the logic, and we will do this in the lending contract as well. 
We do not want anybody to just mint and burn these, so we will implement 
the [Ownable](/smart-contracts/ownable) extension in our NFT. The mint and burn 
logic will be covered differently, we will not be using the mintable and 
burnable extensions.

The `LoanContract` will contain several methods defined in the `Loan` trait.
These methods are restricted and can be called only by an owner of the contract.
There is not too much logic to split it, so everything will be implemented
in the body of the contract.

## Definition of the `Loan` trait

In the `traits/loan.rs`, we will define a `Loan` trait.
That trait contains three super traits: `PSP34`, `PSP34Metadata`, and `Ownable`.
Also, the trait contains several methods, and the definition of the `LoanInfo`
(that structure is used during interacting with the contract 
so it is defined in the `traits` instead of the body of the contract).
`LoanRef` can be used by other developers to do a cross contract call to `LoanContract`.

```rust
use ink_storage::traits::{
    PackedLayout,
    SpreadLayout,
};
use openbrush::{
    contracts::traits::{
        ownable::*,
        psp34::{
            extensions::metadata::*,
            *,
        },
    },
    traits::{
        AccountId,
        Balance,
        Timestamp,
    },
};

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, Clone, scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
pub struct LoanInfo {
    pub borrower: AccountId,
    pub collateral_token: AccountId,
    pub collateral_amount: Balance,
    pub borrow_token: AccountId,
    pub borrow_amount: Balance,
    pub liquidation_price: Balance,
    pub timestamp: Timestamp,
    pub liquidated: bool,
}

#[openbrush::wrapper]
pub type LoanRef = dyn Loan + PSP34 + PSP34Metadata + Ownable;

#[openbrush::trait_definition]
pub trait Loan: PSP34 + PSP34Metadata + Ownable {
    /// This function initalizes data of a loan and mint token inside it
    #[ink(message)]
    fn create_loan(&mut self, loan_info: LoanInfo) -> Result<(), PSP34Error>;

    /// This function frees data of a loan and burn token inside it
    #[ink(message)]
    fn delete_loan(&mut self, initiator: AccountId, loan_id: Id) -> Result<(), PSP34Error>;

    /// This function will be used when the user repays their loan only partially
    #[ink(message)]
    fn update_loan(
        &mut self,
        loan_id: Id,
        new_borrow_amount: Balance,
        new_timestamp: Timestamp,
        new_collateral_amount: Balance,
    ) -> Result<(), PSP34Error>;

    /// This function will set a loan to liquidated
    #[ink(message)]
    fn liquidate_loan(&mut self, loan_id: Id) -> Result<(), PSP34Error>;

    /// Function returns `LoanInfo` by `Id`
    #[ink(message)]
    fn get_loan_info(&self, loan_id: Id) -> Result<LoanInfo, PSP34Error>;
}

```

## Add dependencies

In addition to the dependencies imported in the [PSP-34](/smart-contracts/PSP34)
documentation, we will also add the `ownable` dependency the same way as in the
[ownable](/smart-contracts/ownable) documentation. We will be using `LoanContract`
as a dependency in our lending contract to instantiate it. So we need to also add
the `"rlib"` crate type to have the ability to import the `LoanContract` as a dependency.

## Implement the contract

We want a basic [PSP-34](/smart-contracts/PSP34) token with metadata and ownable extensions, 
so we will add these to our contract. We will add a `openbrush::contract` macro to our contract and add some imports:

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

/// This contract will represent the loan of a user
#[openbrush::contract]
pub mod loan {
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::{
            ownable::*,
            psp34::extensions::metadata::*,
        },
        storage::Mapping,
    };

    use openbrush::modifiers;

    use ink_prelude::{
        string::String,
        vec::Vec,
    };
    use lending_project::traits::loan::*;
```

## Define the storage

We will derive the storage traits related to `PSP-34`, `PSP-34 Metadata`, and 
`Ownable` and declare the fields related to these traits. Also, we will declare 
fields related to `Loan` itself.

```rust
/// Define the storage for PSP34 data, Metadata data and Ownable data
#[ink(storage)]
#[derive(SpreadAllocate, PSP34Storage, OwnableStorage, PSP34MetadataStorage)]
pub struct LoanContract {
    #[PSP34StorageField]
    psp34: PSP34Data,
    #[OwnableStorageField]
    ownable: OwnableData,
    #[PSP34MetadataStorageField]
    metadata: PSP34MetadataData,

    // Fields of current contract
    /// mapping from token id to `LoanInfo`
    loan_info: Mapping<Id, LoanInfo>,
    /// the id of last loan
    last_loan_id: Id,
    /// ids no longer used (can be reused)
    freed_ids: Vec<Id>,
}
```

## Implement the extension traits

We will be using these extensions in our NFT token, so we will implement them for our storage.

```rust
/// implement PSP34 Trait for our NFT
impl PSP34 for LoanContract {}

/// implement Ownable Trait for our NFT
impl Ownable for LoanContract {}

/// implement PSP34Metadata Trait for our NFT
impl PSP34Metadata for LoanContract {}
```

## Implement the Loan trait

We will implement the `Loan` trait. 
All functions except one are restricted by the `only_owner` modifier.

```rust
impl Loan for LoanContract {
    #[modifiers(only_owner)]
    #[ink(message)]
    fn create_loan(&mut self, mut loan_info: LoanInfo) -> Result<(), PSP34Error> {
        let loan_id = self._get_next_loan_id_and_increase()?;
        if self.loan_info.get(&loan_id).is_some() {
            return Err(PSP34Error::Custom(String::from("This loan id already exists!")))
        }
        loan_info.liquidated = false;
        self.loan_info.insert(&loan_id, &loan_info);
        self._mint_to(loan_info.borrower, loan_id)
    }

    #[modifiers(only_owner)]
    #[ink(message)]
    fn delete_loan(&mut self, initiator: AccountId, loan_id: Id) -> Result<(), PSP34Error> {
        self.loan_info.remove(&loan_id);
        self._burn_from(initiator, loan_id)
    }

    #[modifiers(only_owner)]
    #[ink(message)]
    fn update_loan(
        &mut self,
        loan_id: Id,
        new_borrow_amount: Balance,
        new_timestamp: Timestamp,
        new_collateral_amount: Balance,
    ) -> Result<(), PSP34Error> {
        self._update_loan(loan_id, new_borrow_amount, new_timestamp, new_collateral_amount)
    }

    #[modifiers(only_owner)]
    #[ink(message)]
    fn liquidate_loan(&mut self, loan_id: Id) -> Result<(), PSP34Error> {
        self._liquidate_loan(loan_id)
    }

    #[ink(message)]
    fn get_loan_info(&self, loan_id: Id) -> Result<LoanInfo, PSP34Error> {
        let loan_info = self.loan_info.get(&loan_id);
        if loan_info.is_none() {
            return Err(PSP34Error::Custom(String::from("Loan does not exist")))
        }
        Ok(loan_info.unwrap())
    }
}
```

## Define the constructor and add functions

Finally, we will define the constructor where we will set the name and
the symbol of the token and then initialize the owner of the token
(that owner will be able to mint and burn the tokens).
We will also add several helper functions.

```rust
impl LoanContract {
    /// constructor with name and symbol
    #[ink(constructor, payable)]
    pub fn new() -> Self {
        ink_lang::codegen::initialize_contract(|instance: &mut LoanContract| {
            instance.last_loan_id = Id::U8(1u8);
            instance.freed_ids = Vec::new();
            instance._set_attribute(
                Id::U8(1u8),
                String::from("LoanContract NFT").into_bytes(),
                String::from("L-NFT").into_bytes(),
            );
        })
    }

    /// internal function to update data of a loan
    fn _update_loan(
        &mut self,
        loan_id: Id,
        new_borrow_amount: Balance,
        new_timestamp: Timestamp,
        new_collateral_amount: Balance,
    ) -> Result<(), PSP34Error> {
        let loan_info = self.loan_info.get(&loan_id);

        if loan_info.is_none() {
            return Err(PSP34Error::Custom(String::from("This loan does not exist!")))
        }

        let mut loan_info = loan_info.unwrap();
        loan_info.collateral_amount = new_collateral_amount;
        loan_info.borrow_amount = new_borrow_amount;
        loan_info.timestamp = new_timestamp;

        self.loan_info.insert(&loan_id, &loan_info);

        Ok(())
    }

    /// internal function to set loan to liquidated
    fn _liquidate_loan(&mut self, loan_id: Id) -> Result<(), PSP34Error> {
        let loan_info = self.loan_info.get(&loan_id);

        if loan_info.is_none() {
            return Err(PSP34Error::Custom(String::from("This loan does not exist!")))
        }

        let mut loan_info = loan_info.unwrap();
        loan_info.liquidated = true;

        self.loan_info.insert(&loan_id, &loan_info);

        Ok(())
    }

    /// internal function to return the id of a new loan and to increase it in the storage
    fn _get_next_loan_id_and_increase(&mut self) -> Result<Id, PSP34Error> {
        if self.freed_ids.len() > 0 {
            return Ok(self.freed_ids.pop().unwrap())
        }
        let current = self.last_loan_id.clone();
        // It is not fully correct implementation of the increasing. but it is only an example
        match current {
            Id::U8(v) => {
                if v == u8::MAX {
                    return Err(PSP34Error::Custom(String::from("Max Id reached!")))
                }
                self.last_loan_id = Id::U8(v + 1);
            }
            _ => {}
        };
        Ok(current)
    }
}
```