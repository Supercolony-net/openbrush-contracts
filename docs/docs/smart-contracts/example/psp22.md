---
sidebar_position: 3
title: Implement PSP-22 token
---

First, we will cover the implementation of [PSP-22](/smart-contracts/PSP22/psp22) token used by our smart contract, which will represent the stable coin that we will be lending and another [PSP-22](/smart-contracts/PSP22/psp22) token which we will be using as collateral. These are used just to test our example, you will not be creating an actual [PSP-22](/smart-contracts/PSP22/psp22) implementation of stable coin or collateral token in your lending protocol, but this will also showcase how to implement a basic implementation of a fungible token with OpenBrush.

## Add dependencies

First we will add the dependencies used in our [PSP-22](/smart-contracts/PSP22/psp22) contract to the `Cargo.toml` file. You will import the same dependencies as in the [PSP-22](/smart-contracts/PSP22/psp22) documentation, so we will not show it here to keep it simple.

## Implement the contract

We want a basic [PSP-22](/smart-contracts/PSP22/psp22) token with metadata, so we will add the [PSP-22 Metadata](/smart-contracts/PSP22/extensions/metadata) extension to our contract. We will add a `brush::contract` macro to our contract and add some imports:

```rust
#[brush::contract]
pub mod token {
    use ink_lang::{
        EmitEvent,
        Env,
    };
    use ink_prelude::string::String;
    use ink_storage::Lazy;
    use psp22::{
        extensions::metadata::*,
        traits::*,
    };
```

## Define the storage

We will derive the storage traits related to `PSP-22` and `PSP-22 Metadata` and declare the fields related to these traits.

```rust
#[ink(storage)]
#[derive(Default, PSP22Storage, PSP22MetadataStorage)]
pub struct Token {
    #[PSP22StorageField]
    psp22: PSP22Data,
    #[PSP22MetadataStorageField]
    metadata: PSP22MetadataData,
}
```

## Define Event structs

Our token will be emitting events on transfer and approval, so to do that, we need to define them and override the functions to emit them. We will define an event for `Approval` and an event for `Transfer`.

```rust
#[ink(event)]
pub struct Approval {
    #[ink(topic)]
    owner: AccountId,
    #[ink(topic)]
    spender: AccountId,
    value: Balance,
}

#[ink(event)]
pub struct Transfer {
    #[ink(topic)]
    from: Option<AccountId>,
    #[ink(topic)]
    to: Option<AccountId>,
    value: Balance,
}
```

We will also override the `_emit_transfer_event` and `_emit_approval_event` functions and emit the events inside them.

```rust
impl PSP22 for Token {
    fn _emit_transfer_event(&self, from: Option<AccountId>, to: Option<AccountId>, amount: Balance) {
        self.env().emit_event(Transfer {
            from: _from,
            to: _to,
            value: _amount,
        });
    }

    fn _emit_approval_event(&self, _owner: AccountId, _spender: AccountId, _amount: Balance) {
        self.env().emit_event(Approval {
            owner: _owner,
            spender: _spender,
            value: _amount,
        });
    }
}
```

## Implement the Metadata trait and define the constructor

We will implement the `PSP22Metadata` trait and define the constructor where we will set the `name` and the `symbol` for our token and we will also mint the initial supply of tokens.

```rust
impl PSP22Metadata for Token {}

impl Token {
    #[ink(constructor)]
    pub fn new(name: Option<String>, symbol: Option<String>) -> Self {
        let mut instance = Self::default();
        Lazy::set(&mut instance.metadata.name, name);
        Lazy::set(&mut instance.metadata.symbol, symbol);
        Lazy::set(&mut instance.metadata.decimals, 18);
        let total_supply = 1_000_000 * 10_u128.pow(18);
        assert!(instance._mint(instance.env().caller(), total_supply).is_ok());
        instance
    }
}
```