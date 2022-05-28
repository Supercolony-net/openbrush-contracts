---
sidebar_position: 4
title: Shares contract
---

Similarly, we will implement another [PSP-22](/smart-contracts/PSP22) token 
which will represent the ownership of assets available by the smart contract 
to be lent. In this token, we will need [PSP-22 Metadata](/smart-contracts/PSP22/extensions/metadata) 
and we will also need to mint and burn this token. We only want our contract(lending contract) to 
perform these actions, so we will also add the [Ownable](/smart-contracts/ownable) extension.

## Definition of the `Shares` trait

In the `traits/shares.rs`, we will define a `Shares` trait.
That trait contains the next super traits: `PSP22`, `PSP22Mintable`, `PSP22Burnable`, `PSP22Metadata`, and `Ownable`, without any other method.
That shows that `Shares` is `PSP22` with mint and burn methods that can be called only by the owner.
In the implementation of the contract, we will implement that trait to be sure that all super traits are also implemented.
`SharesRef` can be used by other developers to do a cross contract call to `SharesContract`.

```rust
use openbrush::contracts::traits::{
    ownable::*,
    psp22::{
        extensions::{
            burnable::*,
            metadata::*,
            mintable::*,
        },
        *,
    },
};

#[openbrush::wrapper]
pub type SharesRef = dyn PSP22 + PSP22Mintable + PSP22Burnable + PSP22Metadata + Ownable;

#[openbrush::trait_definition]
pub trait Shares: PSP22 + PSP22Mintable + PSP22Burnable + PSP22Metadata + Ownable {}
```

## Add dependencies

In addition to the dependencies imported in the [PSP-22](/smart-contracts/PSP22)
documentation, we will also add the `ownable` dependency the same way as in the
[ownable](/smart-contracts/ownable) documentation. We will be using `SharesContract`
as a dependency in our lending contract to instantiate it. So we need to also add
the `"rlib"` crate type to have the ability to import the `SharesContract` as a dependency.

## Implement the contract

Implementing our shares contract will follow the same steps as implementing 
the basic `PSP-22` contract in the previous step, but we will do some small 
changes for the token to be mintable, burnable, and for these functions to 
be restricted. Therefore, on top of the imports in the previous contract, 
we also need these imports:

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

/// This contract will be used to represent the shares of a user
/// and other instance of this contract will be used to represent
/// the amount of borrowed tokens
#[openbrush::contract]
pub mod shares {
    use openbrush::contracts::{
        ownable::*,
        psp22::extensions::{
            burnable::*,
            metadata::*,
            mintable::*,
        },
    };

    use openbrush::modifiers;

    use ink_lang::codegen::Env;

    use ink_prelude::string::String;
    use ink_storage::traits::SpreadAllocate;

    use lending_project::traits::shares::*;
```

## Define the storage

In this storage, we will also derive the storage trait related to `Ownable` 
and declare the field related to this trait.

```rust
/// Define the storage for PSP22 data, Metadata data and Ownable data
#[ink(storage)]
#[derive(Default, SpreadAllocate, PSP22Storage, OwnableStorage, PSP22MetadataStorage)]
pub struct SharesContract {
    #[PSP22StorageField]
    psp22: PSP22Data,
    #[OwnableStorageField]
    ownable: OwnableData,
    #[PSP22MetadataStorageField]
    metadata: PSP22MetadataData,
}
```

## Implement the extension traits

We will be using these extensions in our token, so we will implement them for 
our storage.

```rust
// implement PSP22 Trait for our share
impl PSP22 for SharesContract {}

// implement Ownable Trait for our share
impl Ownable for SharesContract {}

// implement Metadata Trait for our share
impl PSP22Metadata for SharesContract {}

// It forces the compiler to check that you implemented all super traits
impl Shares for SharesContract {}
```

## Implement the Burnable and Mintable traits

Now we will implement the `PSP22Burnable` and `PSP22Mintable` traits. 
These are a little different so we are doing it in a separate section. 
We don't want anybody to mint or burn the tokens, we only want the owner, 
in this case, our lending contract, to do it. So we will add the `PSP22Burnable` 
and `PSP22Mintable` and mark the functions of these traits with the `only_owner` 
restriction.

```rust
// implement Mintable Trait for our share
impl PSP22Mintable for SharesContract {
    /// override the `mint` function to add the `only_owner` modifier
    #[ink(message)]
    #[modifiers(only_owner)]
    fn mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        self._mint(account, amount)
    }
}

// implement Burnable Trait for our share
impl PSP22Burnable for SharesContract {
    /// override the `burn` function to add the `only_owner` modifier
    #[ink(message)]
    #[modifiers(only_owner)]
    fn burn(&mut self, amount: Balance) -> Result<(), PSP22Error> {
        self._burn(self.env().caller(), amount)
    }

    /// override the `burn_from` function to add the `only_owner` modifier
    #[ink(message)]
    #[modifiers(only_owner)]
    fn burn_from(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        self._burn_from(account, amount)
    }
}
```

This will restrict accounts other than the owner of the token (which will be the lending contract) 
from calling these functions.

## Define the constructor

Finally, we will define the constructor where we will set the name and the symbol 
of the token and then initialize the owner of the token 
(which then will be able to mint and burn the tokens).

```rust
impl SharesContract {
    /// constructor with name and symbol
    #[ink(constructor)]
    pub fn new(name: Option<String>, symbol: Option<String>) -> Self {
        ink_lang::codegen::initialize_contract(|instance: &mut SharesContract| {
            let caller = instance.env().caller();
            instance.metadata.name = name;
            instance.metadata.symbol = symbol;
            instance.metadata.decimals = 18;
            instance._init_with_owner(caller);
        })
    }
}
```