---
sidebar_position: 4
title: Shares token
---

Similarly, we will implement another [PSP-22](/smart-contracts/PSP22/psp22) token which will represent the ownership of assets available by the smart contract to be lent. In this token, we will need [PSP-22 Metadata](/smart-contracts/PSP22/extensions/metadata) and we will also need to mint and burn this token. We only want our contract to perform these actions, so we will also add the [Ownable](/smart-contracts/ownable) extension.

## Add dependencies

In addition to the dependencies imported in the [PSP-22](/smart-contracts/PSP22/psp22) documentation, we will also add the `ownable` depdendency the same way as in the [ownable](/smart-contracts/ownable) documentation.

## Implement the contract

Implementing our shares contract will follow the same steps as implementing the basic `PSP-22` contract in the previous step, but we will do some small changes for the token to be mintable, burnable, and for these functions to be restricted. Therefore, on top of the imports in the previous contract, we also need these imports:

```rust
#[brush::contract]
pub mod shares {
    use ownable::traits::*;
    use psp22::extensions::{
        burnable::*,
        metadata::*,
        mintable::*,
    };
```

## Define the storage

In this storage, we will also derive the storage trait related to `Ownable` and declare the field related to this trait.

```rust
#[ink(storage)]
#[derive(Default, PSP22Storage, OwnableStorage, PSP22MetadataStorage)]
pub struct Shares {
    #[PSP22StorageField]
    psp22: PSP22Data,
    #[OwnableStorageField]
    ownable: OwnableData,
    #[PSP22MetadataStorageField]
    metadata: PSP22MetadataData,
}
```

## Implement the extension traits

We will be using these extensions in our token, so we will implement them for our storage.

```rust
impl Ownable for Shares {}

impl PSP22Metadata for Shares {}
```

## Implement the Burnable and Mintable traits

Now we will implement the `PSP22Burnable` and `PSP22Mintable` traits. These are a little different so we are doing it in a separate section. We don't want anybody to mint or burn the tokens, we only want the owner, in this case, our lending contract, to do it. So we will add the `PSP22Burnable` and `PSP22Mintable` and mark the functions of these traits with the `only_owner` restriction.

```rust
impl PSP22Mintable for Shares {
    #[ink(message)]
    #[modifiers(only_owner)]
    fn mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        self._mint(account, amount)
    }
}

impl PSP22Burnable for Shares {
    #[ink(message)]
    #[modifiers(only_owner)]
    fn burn(&mut self, amount: Balance) -> Result<(), PSP22Error> {
        self._burn(self.env().caller(), amount)
    }

    #[ink(message)]
    #[modifiers(only_owner)]
    fn burn_from(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        self._burn_from(account, amount)
    }
}
```

This will restrict accounts other than the owner of the token (which will be the lending contract) from calling these functions.

## Define the constructor

Finally, we will define the constructor where we will set the name and the symbol of the token and then initialize the owner of the token (which then will be able to mint and burn the tokens.)

```rust
impl Shares {
    #[ink(constructor)]
    pub fn new(name: Option<String>, symbol: Option<String>) -> Self {
        let mut instance = Self::default();
        let caller = instance.env().caller();
        Lazy::set(&mut instance.metadata.name, name);
        Lazy::set(&mut instance.metadata.symbol, symbol);
        Lazy::set(&mut instance.metadata.decimals, 18);
        instance._init_with_owner(caller);
        instance
    }
}
```