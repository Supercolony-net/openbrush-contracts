---
sidebar_position: 1
title: Access Control
---

This example shows how you can use the implementation of [access-control](https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/access/access_control) to provide rights for usage of specific smart contract functions.

## Step 1: Import default implementation

With [default `Cargo.toml`](/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush),
you need to import the `access_control` module, enable the corresponding feature, and embed the module data structure
as described in [that section](/smart-contracts/overview#reuse-implementation-of-traits-from-openbrush).

The main trait is `AccessControl`.

## Step 2: Define constructor

Define constructor where you grant `CALLER` role(or any another role) to the caller.

```rust
// You can manually set the number for the role.
// But better to use a hash of the variable name.
// It will generate a unique identifier of this role.
// And will reduce the chance to have overlapping roles.
const CALLER: RoleType = ink::selector_id!("CALLER");

impl Contract {
    #[ink(constructor)]
    pub fn new() -> Self {
        let mut instance = Self::default();

        let caller = Self::env().caller();
        instance._init_with_admin(caller);
        instance.grant_role(CALLER, caller).expect("Should grant CALLER role");
        
        instance
    }
}
```

## Step 3: Customize your contract

Customize it by adding access control logic. We will add a `restricted_function` to `Contract` implementation, 
which will use the `only_role` modifier with `CALLER` parameter, which verifies that the caller has the `CALLER` role. 

```rust
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_access_control {
    use openbrush::{
        contracts::access_control::*,
        modifiers,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        access: access_control::Data,
    }

    // You can manually set the number for the role.
    // But better to use a hash of the variable name.
    // It will generate a unique identifier of this role.
    // And will reduce the chance to have overlapping roles.
    const CALLER: RoleType = ink::selector_id!("CALLER");

    impl AccessControl for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();

            let caller = Self::env().caller();
            instance._init_with_admin(caller);
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

You can check an example of the usage of [Access Control](https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/access_control).