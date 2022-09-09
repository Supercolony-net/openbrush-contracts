---
sidebar_position: 3
title: Proxy
---

This example shows how you can use the implementation of [proxy](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/upgradability/proxy) to to implement proxy pattern for upgradeable contracts.

## Step 1: Include dependencies

Include `openbrush` as dependency in the cargo file or you can use [default `Cargo.toml`](/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush) template.
After you need to enable default implementation of Proxy via `openbrush` features.

```toml
openbrush = { version = "~2.1.0", default-features = false, features = ["proxy"] }
```

## Step 2: Add imports and enable unstable feature

Use `openbrush::contract` macro instead of `ink::contract`. Import **everything** from `openbrush::contracts::ownable` and `openbrush::contracts::proxy`

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_proxy {
    use openbrush::{
        contracts::{
            ownable::*,
            proxy::*,
        },
        modifiers,
    };
...
```

## Step 3: Define storage

Declare storage struct and declare the field related to `ProxyStorage` trait. Then you need to derive the `ProxyStorage` trait and mark the corresponding field with the `#[ProxyStorageField]` attribute. Deriving this trait allows you to reuse the default implementation of `Proxy`.

```rust
use ink_storage::traits::SpreadAllocate;

#[ink(storage)]
#[derive(Default, SpreadAllocate, ProxyStorage)]
pub struct ProxyContract {
    #[ProxyStorageField]
    proxy: ProxyData,
}
```

## Step 4: Inherit logic

Inherit implementation of the `Proxy` trait and of the `Ownable` trait. You can customize (override) methods in this `impl` block.

```rust
impl Ownable for ProxyContract {}

impl Proxy for ProxyContract {}
```

## Step 5: Define constructor

Define the constructor and initialize the owner with the contract initiator. Your basic version of `Proxy` contract is ready!

```rust
impl ProxyContract {
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

## Step 6: Define forward function

Define the forward function to make delegate calls of upgradeable contract through proxy contract.

```rust
impl ProxyContract {
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
        ProxyInternal::_fallback(self);
    }
}
```
## Step 6: Customize your contract

Generally, proxy doesn't need other functionality, but if you need something you can customize it by adding proxy logic. We will add a `proxy_function` to `ProxyContract` implemenation.


You can check an example of the usage of [Proxy](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/proxy).
