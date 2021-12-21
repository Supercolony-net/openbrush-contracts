---
sidebar_position: 2
title: Access Control
---

This example shows how you can use the implementation of [access-control](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/access/access_control) to provide rights for usage of specific smart contract functions.

## Step 1: Include dependencies

Include `brush` as dependency in the cargo file or you can use [default `Cargo.toml`](/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush) template.
After you need to enable a default implementation of Access Control via features of the `brush`.

```toml
brush = { tag = "v1.2.0", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false, features = ["access_control"] }
```

## Step 2: Add imports and enable unstable feature

Use `brush::contract` macro instead of `ink::contract`. Import **everything** from `brush::contracts::access_control`.

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod my_access_control {
    use brush::{
        contracts::access_control::*,
        modifiers,
    };
...
```

## Step 3: Define storage

Declare storage struct and declare the field related to and `AccessControlStorage` trait. Then you need to derive `AccessControlStorage` trait and mark the corresponding field with `#[AccessControlStorageField]` attribute. Deriving this trait allows you to reuse the default implementation of `AccessControl`.

```rust
#[ink(storage)]
#[derive(Default, AccessControlStorage)]
pub struct MyAccessControl {
    #[AccessControlStorageField]
    access: AccessControlData,
}
```

## Step 4: Inherit logic

Inherit implementation of `AccessControl` trait. You can customize (override) functions in this `impl` block.

```rust
impl AccessControl for MyAccessControl {}
```

## Step 5: Define constructor

Define constructor. Your basic version of `AccessControl` contract is ready!

```rust
impl MyAccessControl {
    #[ink(constructor)]
    pub fn new() -> Self {
        Self::default()
    }
}
```

## Step 6: Customize your contract

Customize it by adding access control logic. We will add a `restricted_function` to `MyAccessControl` implemenation, which will use the `only_role` modifier with `CALLER` parameter, which verifies that the caller has the `CALLER` role. Also, we need to update the constructor to grant the `CALLER` role to the caller by default.

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod my_access_control {
    use brush::{
        contracts::access_control::*,
        modifiers,
    };

    #[ink(storage)]
    #[derive(Default, AccessControlStorage)]
    pub struct MyAccessControl {
        #[AccessControlStorageField]
        access: AccessControlData,
    }

    // You can manually set the number for the role.
    // But better to use a hash of the variable name.
    // It will generate a unique identifier of this role.
    // And will reduce the chance to have overlapping roles.
    const CALLER: RoleType = ink_lang::selector_id!("CALLER");

    impl AccessControl for MyAccessControl {}

    impl MyAccessControl {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            let caller = instance.env().caller();
            instance._init_with_admin(caller);
            // We grant counter role to caller in constructor, so they can increase the count
            instance.grant_role(CALLER, caller).expect("Should grant the role");
            instance
        }

        #[ink(message)]
        #[modifiers(only_role(CALLER))]
        pub fn restricted_function(&mut self) -> Result<(), AccessControlError> {
            todo!()
        }
    }
}
```

You can check the example of usage of [Access Control](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/access_control).