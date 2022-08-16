---
sidebar_position: 4
title: PSP22 Wrapper
---

This example shows how you can reuse the implementation of [PSP22](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp22) token with [PSP22 Wrapper](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/token/psp22/extensions/wrapper.rs) extension, which allows you to wrap your `PSP22` token in a `PSP22Wrapper` token which can be used for example for governance.

## Step 1: Include dependencies

Include `openbrush` as dependency in the cargo file or you can use [default `Cargo.toml`](/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush) template.
After you need to enable default implementation of PSP22 via `openbrush` features.

```toml
openbrush = { version = "~2.1.0", default-features = false, features = ["psp22"] }
```

## Step 2: Add imports and enable unstable feature

Use `openbrush::contract` macro instead of `ink::contract`. Import **everything** from `openbrush::contracts::psp22::extensions::wrapper`.

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22_wrapper {
    use openbrush::contracts::psp22::extensions::wrapper::*;
    use ink_storage::traits::SpreadAllocate;
...
```

## Step 3: Define storage

Declare storage struct and declare the fields related to `PSP22Storage` and `PSP22WrapperStorage` traits. Then you need to derive `PSP22Storage` and `PSP22WrapperStorage` traits and mark corresponding fields with `#[PSP22StorageField]` and `#[PSP22WrapperStorageField]` attributes. Deriving these traits allows you to reuse the default implementation of `PSP22` and `PSP22Wrapper`.

```rust
#[ink(storage)]
#[derive(Default, SpreadAllocate, PSP22WrapperStorage, PSP22Storage)]
pub struct MyPSP22Wrapper {
    #[PSP22StorageField]
    psp22: PSP22Data,
    #[PSP22WrapperStorageField]
    wrapper: PSP22WrapperData,
}
```

## Step 4: Inherit logic

Inherit implementations of `PSP22` and `PSP22Wrapper` traits. You can customize (override) methods in this `impl` block.

```rust
impl PSP22 for MyPSP22Wrapper {}

impl PSP22Wrapper for MyPSP22Wrapper {}
```

## Step 5: Define constructor

Define constructor. Your implementation of `PSP22Wrapper` contract is ready!

```rust
impl MyPSP22 {
   #[ink(constructor)]
   pub fn new(token_address: AccountId) -> Self {
        ink_lang::codegen::initialize_contract(|instance: &mut Self| {
            instance._init(token_address);
        })
    }
}
```

You can check an example of the usage of [PSP22 Wrapper](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp22_extensions/wrapper).

You can also check the documentation for the basic implementation of [PSP22](/smart-contracts/PSP22).