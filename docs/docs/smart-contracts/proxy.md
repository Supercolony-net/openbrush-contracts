---
sidebar_position: 3
title: Proxy
---

This example shows how you can use the implementation of [proxy](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/upgradeability/proxy) to to implement proxy pattern for upgradeable contracts.

## Step 1: Import default implementation

With [default `Cargo.toml`](/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush),
you need to import the `proxy` and `ownable` modules, enable the corresponding features, and embed data structures
as described in [that section](/smart-contracts/overview#reuse-implementation-of-traits-from-openbrush).

The main traits are `Ownable` and `Proxy`.

## Step 2: Define constructor

Define the constructor where you initialize the owner with the contract initiator
and passing code hash of the logic layer.

```rust
impl Contract {
    #[ink(constructor)]
    pub fn new(forward_to: Hash) -> Self {
        ink_lang::codegen::initialize_contract(|instance: &mut Self| {
            let caller = instance.env().caller();
            instance._init_with_forward_to(forward_to);
            instance._init_with_owner(caller);
        })
    }
}
```

## Step 3: Define forward function

Define the forward function to make delegate calls of upgradeable contract through proxy contract.

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod proxy {
    use ink_storage::traits::SpreadAllocate;
    use openbrush::contracts::ownable::*;
    use openbrush::contracts::proxy::*;
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct Contract {
        #[storage_field]
        proxy: proxy::Data,
        #[storage_field]
        ownable: ownable::Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(forward_to: Hash) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                let caller = instance.env().caller();
                instance._init_with_forward_to(forward_to);
                instance._init_with_owner(caller);
            })
        }
        #[ink(message, payable, selector = _)]
        pub fn forward(&self) {
            self._fallback()
        }
    }

    impl Ownable for Contract {}

    impl Proxy for Contract {}
}
```

You can check an example of the usage of [Proxy](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/proxy).
